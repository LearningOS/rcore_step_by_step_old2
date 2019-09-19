all: book

book: _book/*
	cp _book/search_index.json ../rcore_step_by_step_webdoc/search_index.json
	cp _book/search_plus_index.json ../rcore_step_by_step_webdoc/search_plus_index.json
	cp -R _book/gitbook/ ../rcore_step_by_step_webdoc/gitbook/
	cp _book/index.html ../rcore_step_by_step_webdoc/index.html
	cp -R _book/docs/ ../rcore_step_by_step_webdoc/docs/
	cd ../rcore_step_by_step_webdoc && \
	git add . && \
	git commit -m "update" && \
	git push