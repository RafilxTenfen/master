
gallon_SOURCES=$(shell find ./cmd/gallon -name "*.go")

gallon: build/$(OS)-$(ARCH)/gallon
	cp $< $@

build/linux-amd64/gallon: $(gallon_SOURCES) $(SOURCES)
	env GOARCH=amd64 GOOS=linux $(GOBUILD) $(FLAGS) $(LDFLAGS) -o $@ ./cmd/gallon

build/darwin-amd64/gallon: $(gallon_SOURCES) $(SOURCES)
	env GOARCH=amd64 GOOS=darwin $(GOBUILD) $(FLAGS) $(LDFLAGS) -o $@ ./cmd/gallon

build/linux-arm7/gallon: $(gallon_SOURCES) $(SOURCES)
	env GOARM=7 GOARCH=arm GOOS=linux $(GOBUILD) $(FLAGS) $(LDFLAGS) -o $@ ./cmd/gallon
dist/linux-amd64.tar.gz: $(addprefix build/linux-amd64/,$(TARGETS))
	mkdir -p dist
	tar -czf $@ -C build/linux-amd64 .
dist/darwin-amd64.tar.gz: $(addprefix build/darwin-amd64/,$(TARGETS))
	mkdir -p dist
	tar -czf $@ -C build/darwin-amd64 .
dist/linux-arm7.tar.gz: $(addprefix build/linux-arm7/,$(TARGETS))
	mkdir -p dist
	tar -czf $@ -C build/linux-arm7 .
