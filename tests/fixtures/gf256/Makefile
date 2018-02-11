
SAGE = sage

TARGETS = gf256_add.txt.gz gf256_sub.txt.gz gf256_div.txt.gz gf256_mul.txt.gz gf256_pow.txt.gz

.PHONY: all clean

all: $(TARGETS)

%.txt.gz: %.sage common.py
	$(SAGE) $< | gzip > $@

common.py: common.sage
	$(SAGE) --preparse $<
	mv common.sage.py common.py

clean:
	$(RM) $(TARGETS) common.py
