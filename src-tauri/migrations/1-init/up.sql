CREATE TABLE "documents" (
	"hash"	BLOB CHECK(length(32)),
	"document"	BLOB NOT NULL,
	"filename" TEXT,
	PRIMARY KEY("hash")
);