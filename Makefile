all: book

book: _book/*
	cp -R _book/ ../rcore_step_by_step_webdoc
	cd ../rcore_step_by_step_webdoc && \
	git add . && \
	git commit -m "update" && \
	git push