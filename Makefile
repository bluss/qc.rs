
SRCS = qc.rs lazy.rs shrink.rs arbitrary.rs

qc: $(SRCS)
	rust build --test $<

libqc: $(SRCS)
	rustc $<

test: qc
	./qc

.PHONY: test
