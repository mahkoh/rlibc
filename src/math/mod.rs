//! implementation inspired by dietlibc
//! this needs unit tests
//! some of the assembly could be rewritten in Rust

use types::{size_t, int_t};
use rust::prelude::*;

type f128 = f64;

const __halff: f32 = 0.5;
const __half: f64 = 0.5; // == (f64)0x3f000000
const M_1_SQRT2PI: f64 = 0.398942280401432686;
/* even function in (0): Coefficients for gamma(0) */

const tab1: &'static [f64] = &[
0.398942280401432677926,        -0.066490380066905446321,       9.97355701003581694794E-3,      -1.18732821548045439878E-3,     1.15434687616155288764E-4,      -9.44465625950361453450E-6,     6.65969351631665127484E-7,      -4.12266741486268888409E-8,     2.27352982437280636972E-9,      -1.13011716416192129505E-10
]; // len = 9 + 1

/* non even or odd function in (x), x>0: Coefficients for gamma(x), x>0 */

const tab2: &'static [[f64; 32]] = &[
[ -0.158655253931457051468,      0.241970724519143349823,       -0.120985362259571674911,        0.,                             0.0201642270432619458197,      -4.03284540865238916394E-3,     -2.01642270432619458197E-3,      7.68161030219502697887E-4,      1.20025160971797296538E-4,     -8.80184513793180174807E-5,     -1.86705805956129127862E-6,      7.37124220917704609315E-6,     -4.72826391707080259142E-7,     -4.83395817951682973566E-7,      6.57036391970156141055E-8,      2.5544260402922190768E-8,      -5.4292285616752144141E-9,      -1.08932444506260820153E-9,      3.44399256708718202774E-10,     3.6021429664641554881E-11,     -1.81147204852239925966E-11,    -7.66935128389784976374E-13,     8.19047721646461768154E-13,    -3.78144699611990981391E-15,    -3.24856460059989147863E-14,     1.44438130842455313227E-15,     1.14391687912824634892E-15,    -9.38053726039148625184E-17,    -3.59908648108845288945E-17,     4.36020846676166022246E-18,     1.01298640134330880603E-18,    -1.68640470512244526894E-19 ],
[ -0.0227501319481792072104,     0.0539909665131880519553,      -0.0539909665131880519553,       0.0269954832565940259776,      -4.49924720943233766301E-3,     -2.24962360471616883129E-3,      1.34977416282970129877E-3,     -1.17837426913704081544E-4,     -1.15159303574756261652E-4,      3.70473728554448438507E-5,      2.82690796888936559912E-6,     -3.54513195524355369855E-6,      3.76695631261094890352E-7,      1.92024079214184701051E-7,     -5.22690859049557191018E-8,     -4.91799344974114749666E-9,      3.66377919234006038965E-9,     -1.5981997209104676352E-10,     -1.73812379171063320997E-10,     2.62403075313043113473E-11,     5.60918720760414713346E-12,    -1.72126983666416144614E-12,    -8.63428809787622525331E-14,     7.89441765474563834480E-14,    -3.13747960081562321348E-15,    -2.77519506625391157547E-15,     3.29321944203493138076E-16,     7.44375150395529134369E-17,    -1.66428523299294690222E-17,    -1.32735612757620496568E-18,     6.24122437514304644794E-19,     1.12471123532438919306E-21 ],
[ -1.3498980316300945272E-3,     4.43184841193800717687E-3,     -6.64777261790701076574E-3,      5.90913121591734290293E-3,     -3.32388630895350538287E-3,      1.10796210298450179421E-3,     -1.10796210298450179595E-4,     -8.44161602273906129349E-5,      4.35270826172482847927E-5,     -6.30190085030867423515E-6,     -1.9785037553294674925E-6,       1.05520200284238266374E-6,     -1.13913852579575399458E-7,     -4.81174572974454799623E-8,      1.78216871733806513653E-8,     -5.85637697215219690327E-10,    -9.29791350219350980904E-10,     1.96377023046901260016E-10,     1.58870373467897094393E-11,    -1.22699105512396660364E-11,     1.08794270836433192571E-12,     3.99646995170699427940E-13,    -1.01594404465456044793E-13,    -3.33469605506835759271E-15,     4.46588935876766499879E-15,    -4.08076707607833277747E-16,    -1.17808602368979218862E-16,     2.76224909899945482352E-17,     1.09206599392049874162E-18,    -1.03145418746203977253E-18,     6.79984672177279963209E-20,     2.55831283729070534712E-20 ],
[ -3.16712418331199212695E-5,    1.33830225764885351832E-4,     -2.67660451529770703664E-4,      3.34575564412213379613E-4,     -2.89965489157251595673E-4,      1.8178605666396926958E-4,      -8.25286392216793003064E-5,      2.55180251904870680833E-5,     -3.91665839292075186649E-6,     -7.40182052221464123606E-7,      6.44220233592652481453E-7,     -1.73701553397390201613E-7,      9.09595464817154590424E-9,      9.44943118114780783705E-9,     -3.29957075383376125942E-9,      2.94920746951281580686E-10,     1.18744773902482360274E-10,    -4.42039585809856402486E-11,     3.61422484008923382324E-12,     1.43638335494248833511E-12,    -4.58476794992724591068E-13,     2.23496663226445199624E-14,     1.57839046076890756440E-14,    -3.67258220998453293248E-15,    -1.69716269032291432153E-17,     1.43497778353923791279E-16,    -2.14499365995613073838E-17,    -1.93255135682867953692E-18,     1.01377499752128183701E-18,    -7.55713215369572830154E-20,    -2.25510650946079103289E-20,     5.26633993110171917109E-21 ],
[ -2.86651571879193912033E-7,    1.48671951473429770924E-6,     -3.7167987868357442731E-6,       5.9468780589371908374E-6,      -6.81413110919886450076E-6,      5.92209940035828587496E-6,     -4.02653201907205629582E-6,      2.17108246596119665457E-6,     -9.25512396325170449452E-7,      3.03096091545533908077E-7,     -6.92802772105295808398E-8,      6.69226396924248971087E-9,      2.46006252876483997508E-9,     -1.41806830376639605249E-9,      3.44251040657349801884E-10,    -2.6965166176434937652E-11,     -1.16546962748761528049E-11,     4.91490145086991326748E-12,    -7.55854519365765424197E-13,    -4.53988828124843593484E-14,     4.71533558309731405623E-14,    -9.17323049919073092370E-15,     4.35542982587998484108E-17,     3.71238868922011013332E-16,    -7.90772907386322623053E-17,     1.58463483904927528072E-18,     2.61503941976309571331E-18,    -5.40699423853895351239E-19,     6.61825040533797444037E-21,     1.68378440730394776550E-20,    -3.01930850797704474581E-21,    -3.80658085177617928332E-23 ],
[ -9.8658764503769814198E-10,    6.07588284982328549581E-9,     -1.82276485494698564874E-8,      3.54426499573024987263E-8,     -5.01260335110421053478E-8,      5.48348427196551516061E-8,     -4.81513715848495375522E-8,      3.47446467489597046263E-8,     -2.08994095347716137282E-8,      1.0554987922587771203E-8,      -4.4752674615729637229E-9,       1.57746505810079893253E-9,     -4.49697115294871911476E-10,     9.63210042443717269402E-11,    -1.16300711402336909847E-11,    -1.31070037808191623761E-12,     1.16993345829435057496E-12,    -3.40636420312606285351E-13,     5.23724821541706939045E-14,     3.93541148139975862961E-16,    -2.59886413069218394637E-15,     7.24729556829529838503E-16,    -8.51485747763574768020E-17,    -7.86503719948806184368E-18,     5.35986191777031053618E-18,    -9.84873767617830925356E-19,     2.93759678710573738811E-20,     2.85458592629073152182E-20,    -7.12725445137377009753E-21,     5.25419393758902871947E-22,     1.24299023131490990316E-22,    -4.04419210566489645405E-23 ],
[ -1.27981254388583500631E-12,   9.1347204083645933588E-12,     -3.19715214292760767584E-11,     7.30777632669167468738E-11,    -1.22557498812224960902E-10,     1.60618833847077433236E-10,    -1.71047639646627010648E-10,     1.51926349902927316213E-10,    -1.14609023345779936276E-10,     7.43697341394886835864E-11,    -4.18713451557949730558E-11,     2.05606050331840905587E-11,    -8.82161466664564577599E-12,     3.30031395277698236679E-12,    -1.06851205331295409813E-12,     2.94333808755089195146E-13,    -6.64411715537625335642E-14,     1.11264855981436243262E-14,    -8.52918435682649455145E-16,    -2.38837813662069487819E-16,     1.23994634366691956599E-16,    -3.05269770279941723219E-17,     4.34539596489459676621E-18,    -5.55819387468189608390E-20,    -1.56974672263484202926E-19,     4.60835492190702561464E-20,    -6.61112150617493330405E-21,     7.28424268476803924831E-23,     2.09156005934313228089E-22,    -5.29080328670107625978E-23,     5.61375000671507211726E-24,     3.82199410465700894394E-25 ],
[ -6.22096057427178413283E-16,   5.05227108353689229741E-15,    -2.02090843341475691883E-14,     5.30488463771373691202E-14,    -1.02729512031916810045E-13,     1.56409892294496290711E-13,    -1.94849254788406146283E-13,     2.04064637342166989709E-13,    -1.83187931471980616892E-13,     1.42994099344605424348E-13,    -9.8111907789286062426E-14,      5.96545975367403288587E-14,    -3.23370114040930933005E-14,     1.56932853967230342257E-14,    -6.83548101324218922896E-15,     2.67410077774155118457E-15,    -9.38313996431647887562E-16,     2.94090734842381109313E-16,    -8.16448235152204729921E-17,     1.9758222496699617607E-17,     -4.03590262164308783690E-18,     6.43662361965717426956E-19,    -5.93446415094778572090E-20,    -6.07164564350191039536E-21,     4.38906686886388095825E-21,    -1.17175498170220204828E-21,     1.98482140750318604418E-22,    -1.70803571702439545981E-23,    -1.94600332107885234554E-24,     1.10477141319981582738E-24,    -2.31975718243847439962E-25,     2.54148402104633283670E-26 ],
[ -1.12858840595384064928E-19,   1.02797735716689148111E-18,    -4.62589810725101166456E-18,     1.37063647622252197466E-17,    -3.0068337697131575822E-17,      5.2067053140503053517E-17,     -7.40914680178037035E-17,        8.9062000172830588611E-17,     -9.22563786210983011008E-17,     8.35975730487397716492E-17,    -6.70372487553237232779E-17,     4.80088566412770650047E-17,    -3.09280630297969106245E-17,     1.8026496052333452774E-17,     -9.54924880090907168481E-18,     4.61362333444861021959E-18,    -2.03812361224098073479E-18,     8.24578860830779678155E-19,    -3.0572087552697254564E-19,      1.03827313453936543577E-19,    -3.22407758977306397999E-20,     9.12052549039695437376E-21,    -2.33541947993595580264E-21,     5.35339963891271164659E-22,    -1.07674173853083520575E-22,     1.82413373046113374293E-23,    -2.33864726317468746329E-24,     1.29928813344150027051E-25,     3.86668349205203745336E-26,    -1.63203452712600670685E-26,     3.65165372186699607411E-27,    -5.51243539825332137371E-28 ],
[ -7.61985302416052609616E-24,   7.69459862670641937159E-23,    -3.84729931335320968601E-22,     1.26960877340655919637E-21,    -3.10990027829384449637E-21,     6.02935924057670511377E-21,    -9.6342786971886625897E-21,      1.30454744197246721374E-20,    -1.52745988785284834672E-20,     1.57034665186695273938E-20,    -1.43457243961336621961E-20,     1.17567385540485497556E-20,    -8.7104848256363928121E-21,      5.87137214731944288587E-21,    -3.61951956727412561213E-21,     2.04954715001535632502E-21,    -1.06982832733527370879E-21,     5.1628428354196120786E-22,     -2.30885865897937993512E-22,     9.58556229281154921137E-23,    -3.69911125531027884646E-23,     1.32784897023484841369E-23,    -4.43433027366044567275E-24,     1.37688611947822111040E-24,    -3.96971995397574368025E-25,     1.06008163579031271153E-25,    -2.61149430849477426613E-26,     5.89698164189548613154E-27,    -1.20793190886658723050E-27,     2.20446342551066852143E-28,    -3.46061447029252398335E-29,     4.28913922246949096952E-30 ]
]; // [] [31 + 1]

/// double[8]
const tab3: &'static [f64] = &[ 1., -1., 3., -15., 105., -945., 10395., -135135.0 ];

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn ldexp(x: f64, exp: int_t) -> f64 {
	let ret: f64;
	unsafe { asm!{
	"	fildl $2
		flds $1
		fscale
	" : "={st}"(ret) : "m"(x), "m"(exp) :: "volatile"
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn ldexpf(x: f32, exp: int_t) -> f32 {
	let ret: f32;
	unsafe { asm!{
	"	fildl $2
		flds $1
		fscale
	" : "={st}"(ret) : "m"(x), "m"(exp) :: "volatile"
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn floorf(x: f32) -> f32
{
	let ret: f32;
	unsafe { asm!{
	   "flds $1
		movb $$0x04,%ah

		xorl %ecx,%ecx	# wyzerowanie rejestru
		movb %ah,%ch	# i utworzenie maski w cx
		pushq %rax	# krotsze niz subl $$4,%esp
		fstcw (%esp)
		movw (%esp),%ax
		andb $$0x03,%ah	# wyzerowanie bitow 11 i 10
		orl %ecx,%eax	# ustawienie bitow z maski
		movw %ax,2(%esp)
		fldcw 2(%esp)
		frndint
		fldcw (%esp)	# odtworzenie rejestru sterowania
		popq %rax	# i polozenia stosu
		" : "={st}"(ret) : "m"(x) :: "volatile"
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn floor(x: f64) -> f64
{
	let ret: f64;
	unsafe { asm!{
	   "fldl $1
		movb $$0x04,%ah

		xorl %ecx,%ecx	# wyzerowanie rejestru
		movb %ah,%ch	# i utworzenie maski w cx
		pushq %rax	# krotsze niz subl $$4,%esp
		fstcw (%esp)
		movw (%esp),%ax
		andb $$0x03,%ah	# wyzerowanie bitow 11 i 10
		orl %ecx,%eax	# ustawienie bitow z maski
		movw %ax,2(%esp)
		fldcw 2(%esp)
		frndint
		fldcw (%esp)	# odtworzenie rejestru sterowania
		popq %rax	# i polozenia stosu
		" : "={st}"(ret): "m"(x) :: "volatile"
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn ceilf(x: f32) -> f32
{
	let ret: f32;
	unsafe { asm!{
	   "flds $1
		movb $$0x08,%ah

		xorl %ecx,%ecx	# wyzerowanie rejestru
		movb %ah,%ch	# i utworzenie maski w cx
		pushq %rax	# krotsze niz subl $$4,%esp
		fstcw (%esp)
		movw (%esp),%ax
		andb $$0x03,%ah	# wyzerowanie bitow 11 i 10
		orl %ecx,%eax	# ustawienie bitow z maski
		movw %ax,2(%esp)
		fldcw 2(%esp)
		frndint
		fldcw (%esp)	# odtworzenie rejestru sterowania
		popq %rax	# i polozenia stosu
		" : "={st}"(ret) : "m"(x) :: "volatile"
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn ceil(x: f64) -> f64
{
	let ret: f64;
	unsafe { asm!{
	   "fldl $1
		movb $$0x08,%ah

		xorl %ecx,%ecx	# wyzerowanie rejestru
		movb %ah,%ch	# i utworzenie maski w cx
		pushq %rax	# krotsze niz subl $$4,%esp
		fstcw (%esp)
		movw (%esp),%ax
		andb $$0x03,%ah	# wyzerowanie bitow 11 i 10
		orl %ecx,%eax	# ustawienie bitow z maski
		movw %ax,2(%esp)
		fldcw 2(%esp)
		frndint
		fldcw (%esp)	# odtworzenie rejestru sterowania
		popq %rax	# i polozenia stosu
		" : "={st}"(ret) : "m"(x) :: "volatile"
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn truncf(x: f32) -> f32
{
	let ret: f32;
	unsafe { asm!{
	   "flds $1
		movb $$0x0c,%ah

		xorl %ecx,%ecx	# wyzerowanie rejestru
		movb %ah,%ch	# i utworzenie maski w cx
		pushq %rax	# krotsze niz subl $$4,%esp
		fstcw (%esp)
		movw (%esp),%ax
		andb $$0x03,%ah	# wyzerowanie bitow 11 i 10
		orl %ecx,%eax	# ustawienie bitow z maski
		movw %ax,2(%esp)
		fldcw 2(%esp)
		frndint
		fldcw (%esp)	# odtworzenie rejestru sterowania
		popq %rax	# i polozenia stosu
		" : "={st}"(ret) : "m"(x) :: "volatile"
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn trunc(x: f64) -> f64
{
	let ret: f64;
	unsafe { asm!{
	   "fldl $1
		movb $$0x0c,%ah

		xorl %ecx,%ecx	# wyzerowanie rejestru
		movb %ah,%ch	# i utworzenie maski w cx
		pushq %rax	# krotsze niz subl $$4,%esp
		fstcw (%esp)
		movw (%esp),%ax
		andb $$0x03,%ah	# wyzerowanie bitow 11 i 10
		orl %ecx,%eax	# ustawienie bitow z maski
		movw %ax,2(%esp)
		fldcw 2(%esp)
		frndint
		fldcw (%esp)	# odtworzenie rejestru sterowania
		popq %rax	# i polozenia stosu
		" : "={st}"(ret) : "m"(x) :: "volatile"
	}};
	ret
}

#[no_mangle]
/// TODO eliminate branch
pub fn roundf(x: f32) -> f32
{
	if x >= 0. {
		floorf(x)
	} else {
		ceilf(x)
	}
}

#[no_mangle]
pub fn round(x: f64) -> f64
{
	if x >= 0. {
		floor(x)
	} else {
		ceil(x)
	}
}

/// x * y +z
#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn fmaf(x: f32, y: f32, z: f32) -> f32
{
	let ret: f32;
	unsafe { asm!{
	   "flds $1
		fmuls $2
		fadds $3" : "={st}"(ret) : "m"(x), "m"(y), "m"(z)
	}};
	ret
}

/// x * y + z
#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn fma(x: f64, y: f64, z: f64) -> f64
{
	let ret: f64;
	unsafe { asm!{
	   "fldl $1
		fmull $2
		faddl $3" : "={st}"(ret) : "m"(x), "m"(y), "m"(z)
	}};
	ret
}

/// y % x
#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn fmodf(x: f32, y: f32) -> f32
{
	let ret: f32;
	unsafe { asm!{
	   "flds $2		# y
		flds $1		# x
	.Lfmodf:
		fprem
		fstsw %ax
		sahf
		jp .Lfmodf
		fstp %st(1)" : "={st}"(ret) : "m"(x), "m"(y) :: "volatile"
	}};
	ret
}

/// y % x
#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn fmod(x: f64, y: f64) -> f64
{
	let ret: f64;
	unsafe { asm!{
	   "fldl $2
		fldl $1
	.Lfmod:
		fprem
		fstsw %ax
		sahf
		jp .Lfmod
		fstp %st(1)" : "={st}"(ret) : "m"(x), "m"(y) :: "volatile"
	}};
	ret
}

/// max(x-y,0)
#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn fdimf(x: f32, y: f32) -> f32
{
	let ret: f32;
	unsafe { asm!{
	   "flds $1
		fsubl $2
		fstsw %ax
		sahf
		jnc 1f
		fldz
	1:	nop" : "={st}"(ret) : "m"(x), "m"(y) :: "volatile"
	}};
	ret
}

/// max(x-y,0)
#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn fdim(x: f64, y: f64) -> f64
{
	let ret: f64;
	unsafe { asm!{
	   "fldl $1
		fsubl $2
		fstsw %ax
		sahf
		jnc 1f
		fldz
	1:	nop" : "={st}"(ret) : "m"(x), "m"(y) :: "volatile"
	}};
	ret
}

#[no_mangle]
pub fn fmaxf(x: f32, y: f32) -> f32
{
	if x.is_nan() {
		y
	} else if y.is_nan() {
		x
	} else if x.is_negative() != y.is_negative() {
		/* handle signed zeroes, see C99 Annex F.9.9.2 */
		if x.is_negative() { y } else { x }
	} else {
		if x < y { y } else { x }
	}
}

#[no_mangle]
pub fn fmax(x: f64, y: f64) -> f64
{
	if x.is_nan() {
		y
	} else if y.is_nan() {
		x
	} else if x.is_negative() != y.is_negative() {
		/* handle signed zeroes, see C99 Annex F.9.9.2 */
		if x.is_negative() { y } else { x }
	} else {
		if x < y { y } else { x }
	}
}

#[no_mangle]
pub fn fminf(x: f32, y: f32) -> f32
{
	if x.is_nan() {
		y
	} else if y.is_nan() {
		x
	} else if x.is_negative() != y.is_negative() {
		/* handle signed zeroes, see C99 Annex F.9.9.2 */
		if x.is_negative() { x } else { y }
	} else {
		if x < y { x } else { y }
	}
}

#[no_mangle]
pub fn fmin(x: f64, y: f64) -> f64
{
	if x.is_nan() {
		y
	} else if y.is_nan() {
		x
	} else if x.is_negative() != y.is_negative() {
		/* handle signed zeroes, see C99 Annex F.9.9.2 */
		if x.is_negative() { x } else { y }
	} else {
		if x < y { x } else { y }
	}
}

/* Roots */

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn sqrtf(x: f32) -> f32
{
	let ret: f32;
	unsafe { asm!{
	   "fsqrt" : "={st}"(ret) : "{st}"(x)
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn sqrt(x: f64) -> f64
{
	let ret: f64;
	unsafe { asm!{
	   "fsqrt" : "={st}"(ret) : "{st}"(x)
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn cbrtf(x: f32) -> f32
{
	unsafe { asm!{
	   "flds $0

	# fldt 1/3
		push $$0x3ffd
		pushq 12297829382473034411 # $$0xaaaaaaaaaaaaaaab
		# pushl $$0x00003ffd	# yes, this method of loading 1/3
		# pushl $$0xaaaaaaaa	# is shorter than keeping the data
		# pushl $$0xaaaaaaab	# separate
		fldt (%esp)
		addl $$12,%esp
		fxch			# st(0)=x, st(1)=1/3
		ftst
		fstsw %ax
		sahf
		jz 1f
		jnc finpow
		fchs
		call finpow
		fchs
	1:	ret" :: "m" (x) :: "volatile"
	}};
	unsafe { asm!{
	   "finpowf:
		fyl2x

		fst	%st(1)
		frndint
		fst	%st(2)
		fsubrp
		f2xm1
		fld1
		faddp
		fscale
		ret" :::: "volatile"
	}};
	x // dead code
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn cbrt(x: f64) -> f64
{
	unsafe { asm!{
	   "fldl $0

	# fldt 1/3
		push $$0x3ffd
		pushq 12297829382473034411 # $$0xaaaaaaaaaaaaaaab
		# pushl $$0x00003ffd	# yes, this method of loading 1/3
		# pushl $$0xaaaaaaaa	# is shorter than keeping the data
		# pushl $$0xaaaaaaab	# separate
		fldt (%esp)
		addl $$12,%esp
		fxch			# st(0)=x, st(1)=1/3
		ftst
		fstsw %ax
		sahf
		jz 2f
		jnc finpow
		fchs
		call finpow
		fchs
	2:	ret" :: "m" (x) :: "volatile"
	}};
	unsafe { asm!{
	   "finpow:
		fyl2x

		fst	%st(1)
		frndint
		fst	%st(2)
		fsubrp
		f2xm1
		fld1
		faddp
		fscale
		ret" :::: "volatile"
	}};
	x // dead code
}

#[no_mangle]
pub fn hypotf(x: f32, y: f32) -> f32
{
	sqrtf(x*x + y*y)
}

#[no_mangle]
pub fn hypot(x: f64, y: f64) -> f64
{
	sqrt(x*x + y*y)
}

/* Logarithms */

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn logf(x: f32) -> f32
{
	let ret: f32;
	unsafe {asm!{
	   "fldln2
		flds $1
		fyl2x" : "={st}"(ret) : "m"(x)
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn log(x: f64) -> f64
{
	let ret: f64;
	unsafe { asm!{
	   "fldln2
		fldl $1
		fyl2x" : "={st}"(ret) : "m"(x)
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn log2f(x: f32) -> f32
{
	let ret: f32;
	unsafe { asm!{
	   "fld1
		flds $1
		fyl2x" : "={st}"(ret) : "m"(x)
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn log2(x: f64) -> f64
{
	let ret: f64;
	unsafe { asm!{
	   "fld1
		fldl $1
		fyl2x" : "={st}"(ret) : "m"(x)
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn log10f(x: f32) -> f32
{
	let ret: f32;
	unsafe { asm!{
	   "fldlg2
		flds $1
		fyl2x" : "={st}"(ret) : "m"(x)
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn log10(x: f64) -> f64
{
	let ret: f64;
	unsafe { asm!{
	   "fldlg2
		fldl $1
		fyl2x" : "={st}"(ret) : "m"(x)
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn log1pf(x: f32) -> f32
{
	let ret: f32;
	unsafe { asm!{
	   "flds $1
	# Sprawdzenie zakresu parametru
		fst %st(1)
		pushq	$$0x3ed413cc	# sqrt(2)-1-2^(-25)
		fabs
		flds (%esp)
		popq %rax
		fcompp			# porownanie
		fstsw %ax
		fldln2
		fxch
		sahf
	# |x| >= sqrt(2)-1
		jc 1f
		fyl2xp1
		jmp 2f
	1:	fld1		# x = x + 1
		faddp
		fyl2x
	2:	nop" : "={st}"(ret) : "m"(x) : "rax" : "volatile"
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn log1p(x: f64) -> f64
{
	let ret: f64;
	unsafe { asm!{
	   "fldl $1
	# Sprawdzenie zakresu parametru
		fst %st(1)
		pushq	$$0x3ed413cc	# sqrt(2)-1-2^(-25)
		fabs
		flds (%esp)
		popq %rax
		fcompp			# porownanie
		fstsw %ax
		fldln2
		fxch
		sahf
	# |x| >= sqrt(2)-1
		jc 1f
		fyl2xp1
		jmp 2f
	1:	fld1		# x = x + 1
		faddp
		fyl2x
	2:	nop" : "={st}"(ret) : "m"(x) : "rax" : "volatile"
	}};
	ret
}

/* Exponentials */

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn expf(x: f32) -> f32
{
	let ret: f32;
	unsafe { asm!{
	   "fldl2e
		fmuls $1
		fst	%st(1)
		frndint
		fst	%st(2)
		fsubrp
		f2xm1
		fld1
		faddp
		fscale" : "={st}"(ret) : "m"(x) :: "volatile"
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn exp(x: f64) -> f64
{
	let ret: f64;
	unsafe { asm!{
	   "fldl2e
		fmull $1
		fst	%st(1)
		frndint
		fst	%st(2)
		fsubrp
		f2xm1
		fld1
		faddp
		fscale" : "={st}"(ret) : "m"(x) :: "volatile"
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn exp2f(x: f32) -> f32
{
	let ret: f32;
	unsafe { asm!{
	   "flds $1
		fst	%st(1)
		frndint
		fst	%st(2)
		fsubrp
		f2xm1
		fld1
		faddp
		fscale" : "={st}"(ret) : "m"(x) :: "volatile"
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn exp2(x: f64) -> f64
{
	let ret: f64;
	unsafe { asm!{
	   "fldl $1
		fst	%st(1)
		frndint
		fst	%st(2)
		fsubrp
		f2xm1
		fld1
		faddp
		fscale" : "={st}"(ret) : "m"(x) :: "volatile"
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn expm1f(x: f32) -> f32
{
	let ret: f32;
	unsafe { asm!{
	    "fldl2e
		fmuls $1

		# -1 <= st <= 1 ?
	# finem1:
		fst %st(1)	# st(1)=st(0)
		fabs
		fld1
		fcompp
		fstsw %ax
		sahf
	# |x| >= 1
		jc 1f
		f2xm1
		jmp 2f

	1:	# __finexp
		fst	%st(1)
		frndint
		fst	%st(2)
		fsubrp
		f2xm1
		fld1
		faddp
		fscale

		fld1
		fsubrp

	2:	nop" : "={st}"(ret) : "m"(x) :: "volatile"
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn expm1(x: f64) -> f64
{
	let ret: f64;
	unsafe { asm!{
	   "fldl2e					# note: exp(x) = 2^(x*log2(e))
        fmull    $1             # x*log2(e)
        fld      %st(0)         # x*log2(e)                       x*log2(e)
        frndint                 # int(x*log2(e))                  x*log2(e)
        fxch                    # x*log2(e)                       int(x*log2(e))
        fsub     %st(1),%st(0)  # frac(x*log2(e))                 int(x*log2(e))
        f2xm1                   # 2^(fract(x*log2(e)))-1          int(x*log2(e))
        fscale                  # 2^(x*log2(e))-2^int(x*log2(e))  int(x*log2(e))
        fxch                    # int(x*log2(e))                  2^(x*log2(e))-2^int(x*log2(e))
        fld1                    # 1                               int(x*log2(e))                  2^(x*log2(e))-2^int(x*log2(e))
        fscale                  # 2^int(x*log2(e))                int(x*log2(e))                  2^(x*log2(e))-2^int(x*log2(e))
        fstp     %st(1)         # 2^int(x*log2(e))                2^(x*log2(e))-2^int(x*log2(e))
        fld1                    # 1                               2^int(x*log2(e))                2^(x*log2(e))-2^int(x*log2(e))
        fsubrp   %st(1)         # 2^int(x*log2(e))-1              2^(x*log2(e))-2^int(x*log2(e))
        faddp    %st(1)
        " : "={st}"(ret) : "m"(x) :: "volatile"
	}};
	ret
}

/* Powers */

#[no_mangle]
pub fn powf(mant: f32, expo: f32) -> f32
{
	match (mant, expo) {
		/* special cases 0^x */
		(0.0, expo) if expo > 0. => { 0. },
		(0.0, expo) if expo == 0. => { 1. },
		(0.0, expo) if expo < 0. => { 1./mant },
		/* special cases x^n with n is integer */
		(mant, expo) if (expo as i64) as f32 == expo => {
			let es = expo as i64;
			let (mut e, mut m) = if es < 0 {
				(-es as u64, 1./mant)
			} else {
				(es as u64, mant)
			};

			let mut ret: f32 = 1.;

			loop {
				if e&1 == 1 {
					ret *= m;
				}
				e >>= 1;
				if e == 0 {
					break;
				}
				m *= m;
			}
			ret
		}
		/* normal case */
		(mant, expo) => {expf( logf(mant) * expo )}
	}
}

#[no_mangle]
pub fn pow(mant: f64, expo: f64) -> f64
{
	match (mant, expo) {
		/* special cases 0^x */
		(0.0, expo) if expo > 0. => { 0. },
		(0.0, expo) if expo == 0. => { 1. },
		(0.0, expo) if expo < 0. => { 1./mant },
		/* special cases x^n with n is integer */
		(mant, expo) if (expo as i64) as f64 == expo => {
			let es = expo as i64;
			let (mut e, mut m) = if es < 0 {
				(-es as u64, 1./mant)
			} else {
				(es as u64, mant)
			};

			let mut ret: f128 = 1.;

			loop {
				if e&1 == 1 {
					ret *= m;
				}
				e >>= 1;
				if e == 0 {
					break;
				}
				m *= m;
			}
			ret
		}
		/* normal case */
		(mant, expo) => {exp( log(mant) * expo )}
	}
	/*
    if mant == 0. {
        if expo > 0. {
            return 0.;
        }
        else if expo == 0. {
            return 1.;
        }
        else {
            return 1./mant;
        }
    }

    let ret: f128;
	let mut e: u64 = expo as i64 as u64;
    if expo as i64 == e as i64 {

        if (e as i64) < 0 {
            e    = -e;
            mant = 1./mant;
        }

        ret = 1.;

        loop {
            if e & 1 == 1 {
                ret *= mant;
            }
            e >>= 1;
            if ( e == 0 ){
                break;
            }
            mant *= mant;
        }
        return ret;
    }
    return exp( log(mant) * expo );
    */
}

/* Trigonometric functions */

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn sinf(x: f32) -> f32
{
	let ret: f32;
	unsafe { asm!{
	   "flds $1
	1:	fsin
		fnstsw	%ax
		testb	$$0x04, %ah
		je	3f
		fldpi
		fadd	%st
		fxch	%st(1)
	2:	fprem1
		fnstsw	%ax
		testb	$$0x04, %ah
		jne	2b
		fstp	%st(1)
		fsin
	3:	nop" : "={st}"(ret) : "m"(x) :: "volatile"
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn sin(x: f64) -> f64
{
	let ret: f64;
	unsafe { asm!{
	   "fldl $1
	1:	fsin
		fnstsw	%ax
		testb	$$0x04, %ah
		je	3f
		fldpi
		fadd	%st
		fxch	%st(1)
	2:	fprem1
		fnstsw	%ax
		testb	$$0x04, %ah
		jne	2b
		fstp	%st(1)
		fsin
	3:	nop" : "={st}"(ret) : "m"(x) :: "volatile"
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn cosf(x: f32) -> f32
{
	let ret: f32;
	unsafe { asm!{
	   "flds $1
	1:	fcos
		fnstsw	%ax
		testb	$$0x04, %ah
		je 3f
		fldpi
		fadd	%st
		fxch	%st(1)
	2:	fprem1
		fnstsw	%ax
		testb	$$0x04, %ah
		jne	2b
		fstp	%st(1)
		fcos
	3:	nop" : "={st}"(ret) : "m"(x) :: "volatile"
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn cos(x: f64) -> f64
{
	let ret: f64;
	unsafe { asm!{
	   "fldl $1
	1:	fcos
		fnstsw	%ax
		testb	$$0x04, %ah
		je 3f
		fldpi
		fadd	%st
		fxch	%st(1)
	2:	fprem1
		fnstsw	%ax
		testb	$$0x04, %ah
		jne	2b
		fstp	%st(1)
		fcos
	3:	ret" : "={st}"(ret) : "m"(x) :: "volatile"
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn tanf(x: f32) -> f32
{
	let ret: f32;
	unsafe { asm!{
	   "flds $1
	    call	.__fmod2pif
	    fsincos
        fdivrp" : "={st}"(ret) : "m"(x) :: "volatile"
	}};
	unsafe { asm!{"
	.__fmod2pif:
		fldpi
		fadd %st(0)
		fxch
	.Lfmodftanf:
		fprem
		fstsw %ax
		sahf
		jp .Lfmodftanf
		fstp %st(1)
		ret" :::: "volatile"
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn tan(x: f64) -> f64
{
	let ret: f64;
	unsafe { asm!{
	   "fldl $1
	    call	.__fmod2pi
	    fsincos
        fdivrp" : "={st}"(ret) : "m"(x) :: "volatile"
	}};
	unsafe { asm!{"
	.__fmod2pi:
		fldpi
		fadd %st(0)
		fxch
	.Lfmodtan:
		fprem
		fstsw %ax
		sahf
		jp .Lfmodtan
		fstp %st(1)
		ret" :::: "volatile"
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn asinf(x: f32) -> f32
{
	let ret: f32;
	unsafe { asm!{
	   "flds $1
		fld	%st
		fmul	%st
		fld1
		fsubp
		fsqrt
		fpatan" : "={st}"(ret) : "m"(x)
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn asin(x: f64) -> f64
{
	let ret: f64;
	unsafe { asm!{
	   "fldl $1
		fld	%st
		fmul	%st
		fld1
		fsubp
		fsqrt
		fpatan" : "={st}"(ret) : "m"(x)
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn acosf(x: f32) -> f32
{
	let ret: f32;
	unsafe { asm!{
	   "flds $1
		fld	%st
		fmul	%st
		fld1
		fsubp
		fsqrt
		fxch	%st(1)
		fpatan" : "={st}"(ret) : "m"(x)
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn acos(x: f64) -> f64
{
	let ret: f64;
	unsafe { asm!{
	   "fldl $1
		fld	%st
		fmul	%st
		fld1
		fsubp
		fsqrt
		fxch	%st(1)
		fpatan" : "={st}"(ret) : "m"(x)
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn atanf(x: f32) -> f32
{
	let ret: f32;
	unsafe { asm!{
	   "flds $1
		fld1
		fpatan" : "={st}"(ret) : "m"(x)
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn atan(x: f64) -> f64
{
	let ret: f64;
	unsafe { asm!{
	   "fldl $1
		fld1
		fpatan
		ret" : "={st}"(ret) : "m"(x)
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn atan2f(y: f32, x: f32) -> f32
{
	let ret: f32;
	unsafe { asm!{
	   "flds $1		# y??
		flds $2		# x??
		fpatan" : "={st}"(ret) : "m"(y), "m"(x)
	}};
	ret
}

#[no_mangle]
#[cfg(any(target_arch = "i386", target_arch = "x86_64"))]
pub fn atan2(y: f64, x: f64) -> f64
{
	let ret: f64;
	unsafe { asm!{
	   "flds $1		# y??
		flds $2		# x??
		fpatan" : "={st}"(ret) : "m"(y), "m"(x)
	}};
	ret
}

/* Hyperbolic functions */

#[no_mangle]
pub fn acoshf(x: f32) -> f32
{
	logf(x + sqrtf(x*x - 1.))
}

#[no_mangle]
pub fn acosh(x: f64) -> f64
{
	log(x + sqrt(x*x - 1.))
}

#[no_mangle]
pub fn asinhf(x: f32) -> f32
{
	logf(x + sqrtf(x*x + 1.))
}

#[no_mangle]
pub fn asinh(x: f64) -> f64
{
	log(x + sqrt(x*x + 1.))
}

#[no_mangle]
pub fn atanhf(x: f32) -> f32
{
	__halff * logf( (1.+x) / (1.-x) )
}

#[no_mangle]
pub fn atanh(x: f64) -> f64
{
	__half * log( (1.+x) / (1.-x) )
}

#[no_mangle]
pub fn coshf(x: f32) -> f32
{
	let y: f32 = expf(x);	// TODO f128
	(y + 1./y) * __halff
}

#[no_mangle]
pub fn cosh(x: f64) -> f64
{
	let y: f64 = exp(x);	// TODO f128
	(y + 1./y) * __half
}

#[no_mangle]
pub fn sinhf( x: f32 ) -> f32
{
	let y: f32 = expf(x);	// TODO f128
	(y - 1./y) * __halff
}

#[no_mangle]
pub fn sinh( x: f64 ) -> f64
{
	let y: f64 = exp(x);	// TODO f128
	(y - 1./y) * __half
}

#[no_mangle]
pub fn tanhf(x: f32) -> f32
{
	let y: f32 = expf(x + x);	// TODO f128
	(y - 1.) / (y + 1.)
}

#[no_mangle]
pub fn tanh(x: f64) -> f64
{
	let y: f64 = exp(x + x);	// TODO f128
	(y - 1.) / (y + 1.)
}

// TODO Bessel

/* Error and gamma functions */

#[no_mangle]
pub fn gauss(x: f64) -> f64
{
	let i: u64 = (x + 0.5) as u64;
	let y: f64 = (x * x) as f64;

	match i {
		11...150	=> M_1_SQRT2PI * exp(-0.5*y) / x * __poly(1./y, 7, tab3),
		1 ... 10	=> -__poly((x - i as f64), 31, &tab2[i as usize-1]),
		0	=> 0.5 - x * __poly(y, 9, tab1),
		_	=> 0.,
	}
}

#[no_mangle]
pub fn erf(x: f64) -> f64
{
	if x < 0. {-0.5 + gauss(-x)} else {0.5 - gauss(x)}
}

#[no_mangle]
pub fn erfc(x: f64) -> f64
{
	if x < 0. {1.0 - gauss(-x)} else {gauss(x)}
}

/* internals */
pub fn __poly(x: f64, _n: size_t, c: &[f64]) -> f64
{
	// f128
	// go from c[n] to c[0]
	c.iter().rev().fold(0f64, |carry, ci| carry*x + *ci)
}
