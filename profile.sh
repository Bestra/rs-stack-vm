sudo dtrace -c './target/release/stack_vm samples/while.lox' -o out.stacks -n 'profile-997 /execname == "stack_vm"/ { @[ustack(100)] = count(); }'
~/personal/FlameGraph/stackcollapse.pl out.stacks | ~/personal/FlameGraph/flamegraph.pl > graph.svg
