#!/usr/bin/env python3

import json
import fileinput
import os.path
import subprocess

funcs = {}

# Find subnodes

def find_mods(mod):
	return [x for x in mod['inner']['fields'][0]['items']
		if x['inner']['variant'] == 'ModuleItem']

def find_traits(mod):
	return [x for x in mod['inner']['fields'][0]['items']
		if x['inner']['variant'] == 'TraitItem']

def find_impls(mod):
	return [x for x in mod['inner']['fields'][0]['items']
		if x['inner']['variant'] == 'ImplItem']

def find_funcs(mod):
	return [x for x in mod['inner']['fields'][0]['items']
		if x['inner']['variant'] == 'FunctionItem']

def find_methods(impl):
	return [x for x in impl['inner']['fields'][0]['items']
		if x['inner']['variant'] == 'MethodItem']

def find_provided(trait):
	return [x for x in trait['inner']['fields'][0]['items']
		if x['variant'] == 'ProvidedMethod']

# Visit endnodes

def visit_func(func):
	name = func['source']['filename']
	line = func['source']['loline']
	if not name in funcs:
		funcs[name] = []
	funcs[name].append(line)
	print("%s in %s at %i" % (func['name'], name, line))

visit_method = visit_func

def visit_provided(provided):
	name = provided['fields'][0]['source']['filename']
	line = provided['fields'][0]['source']['loline']
	if not name in funcs:
		funcs[name] = []
	funcs[name].append(line)
	print("%s in %s at %i" % (provided['fields'][0]['name'], name, line))

# Visit intermediate nodes

def visit_impl(impl):
	for m in find_methods(impl):
		visit_func(m)

def visit_trait(trait):
	for m in find_provided(trait):
		visit_provided(m)

def visit_mod(mod):
	for f in find_funcs(mod):
		visit_func(f)

	for i in find_impls(mod):
		visit_impl(i)

	for t in find_traits(mod):
		visit_trait(t)

	for m in find_mods(mod):
		visit_mod(m)

# Output

def process_funcs():
	for name, lines in funcs.items():
		with fileinput.FileInput(files=(name), inplace=True) as fi:
			for line in fi:
				if fi.lineno() in lines:
					print("#[no_split_stack]")
				print(line, end="")

def main():
	if not os.path.isfile("libcore/no_split_stack.txt"):
		if not os.path.isfile("doc.json"):
			subprocess.call(['rustdoc', '--no-defaults', '--passes',
				'collapse-docs', '-w', 'json', 'libcore/lib.rs'])
		with open('libcore/no_split_stack.txt', 'w+'): pass
		with open('doc.json') as doc_json_file:
			doc_json = json.load(doc_json_file)
		root_mod = doc_json['crate']['module']
		visit_mod(root_mod)
		process_funcs()

if __name__ == '__main__':
	main()
