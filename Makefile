all: book

book: _book/index.html _book/docs/*
	cp _book/index.html ../rcore_step_by_step_webdoc/index.html
	cp -R _book/docs ../rcore_step_by_step_webdoc/docs
	cd ../rcore_step_by_step_webdoc
	git add .
	git commit -m "update"
	git push