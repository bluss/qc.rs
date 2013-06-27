
qc: qc.rs lazy.rs shrink.rs arbitrary.rs
	rust build --test $<

test: qc
	./qc

.PHONY: test
