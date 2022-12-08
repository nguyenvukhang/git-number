// found from running `git help --all`
#[rustfmt::skip]
pub const GIT_CMD: &[&str] = &[
    "add", "am", "archive", "bisect", "branch", "bundle", "checkout", "modules",
    "cherry-pick", "citool", "clean", "clone", "commit", "describe", "mailinfo",
    "fetch", "format-patch", "gc", "gitk", "grep", "gui", "init", "log", "diff",
    "maintenance", "merge", "mv", "notes", "pull", "push", "range-diff", "show",
    "rebase", "reset", "restore", "revert", "rm", "scalar", "shortlog", "apply",
    "sparse-checkout", "stash", "status", "submodule", "switch", "tag", "hooks",
    "worktree", "config", "fast-export", "fast-import", "filter-branch", "help",
    "mergetool", "pack-refs", "prune", "reflog", "remote", "repack", "difftool",
    "annotate", "blame", "bugreport", "count-objects", "diagnose", "archimport",
    "gitweb", "instaweb", "merge-tree", "rerere", "show-branch",
    "verify-commit", "verify-tag", "version", "whatchanged","fsck", 
    "cvsexportcommit", "cvsimport", "cvsserver", "imap-send", "p4", "replace",
    "quiltimport", "request-pull", "send-email", "svn",
    "checkout-index", "commit-graph", "commit-tree", "hash-object",
    "index-pack", "merge-file", "merge-index", "mktag", "mktree",
    "multi-pack-index", "pack-objects", "prune-packed", "read-tree",
    "symbolic-ref", "unpack-objects", "update-index", "update-ref",
    "write-tree", "cat-file", "cherry", "diff-files", "diff-index", "diff-tree",
    "for-each-ref", "for-each-repo", "get-tar-commit-id", "ls-files",
    "ls-remote", "ls-tree", "merge-base", "name-rev", "pack-redundant",
    "rev-list", "rev-parse", "show-index", "show-ref", "unpack-file", "var",
    "verify-pack", "daemon", "fetch-pack", "http-backend", "send-pack",
    "update-server-info", "check-attr", "check-ignore", "check-mailmap",
    "check-ref-format", "column", "credential", "credential-cache",
    "credential-store", "fmt-merge-msg", "hook", "interpret-trailers",
     "mailsplit", "merge-one-file", "patch-id", "sh-i18n",
    "sh-setup", "stripspace", "attributes", "cli", "ignore", "mailmap",
     "repository-layout", "revisions", "format-bundle",
    "format-chunk", "format-commit-graph", "format-index", "format-pack",
    "format-signature", "protocol-capabilities", "protocol-common",
    "protocol-http", "protocol-pack", "protocol-v2",
];
