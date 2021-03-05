MSA_PAIRWISE_D = ./msa_pairwise_identity
MSA_PAIRWISE_TEST_FILE_D = $(MSA_PAIRWISE_D)/test_files
MSA_PAIRWISE_TEST_FILE_IN = $(MSA_PAIRWISE_TEST_FILE_D)/silly.fa
MSA_PAIRWISE_TEST_FILE_OUT = $(MSA_PAIRWISE_TEST_FILE_D)/silly.OUT.tsv
MSA_PAIRWISE_TEST_FILE_EXPECTED = $(MSA_PAIRWISE_TEST_FILE_D)/silly.expected.tsv

.PHONY: test_msa_pairwise_identity
.PHONY: test_msa_pairwise_similarity
.PHONY: all

all: test_msa_pairwise_identity test_msa_pairwise_similarity

test_msa_pairwise_identity:
	rm "$(MSA_PAIRWISE_TEST_FILE_OUT)"; \
	cargo run --bin msa_pairwise -- \
	  --infile "$(MSA_PAIRWISE_TEST_FILE_IN)" \
	  identity > \
	  "$(MSA_PAIRWISE_TEST_FILE_OUT)" && \
	diff "$(MSA_PAIRWISE_TEST_FILE_OUT)" \
	     "$(MSA_PAIRWISE_TEST_FILE_EXPECTED)"

test_msa_pairwise_similarity:
	rm "$(MSA_PAIRWISE_TEST_FILE_OUT)"; \
	cargo run --bin msa_pairwise -- \
	  --infile "$(MSA_PAIRWISE_TEST_FILE_IN)" \
	  similarity > \
	  "$(MSA_PAIRWISE_TEST_FILE_OUT)" && \
	diff "$(MSA_PAIRWISE_TEST_FILE_OUT)" \
	     "$(MSA_PAIRWISE_TEST_FILE_EXPECTED)"
