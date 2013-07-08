
SRCS = qc.rs lazy.rs shrink.rs arbitrary.rs

qc: $(SRCS)
	rust build --test $<

libqc: $(SRCS)
	rust build --lib $<

test: qc
	./qc

.PHONY: test
