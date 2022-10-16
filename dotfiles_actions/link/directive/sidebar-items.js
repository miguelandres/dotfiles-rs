window.SIDEBAR_ITEMS = {"constant":[["CREATE_PARENT_DIRS_SETTING","Create parent dirs if they don’t exist"],["DIRECTIVE_NAME","Name of the link directive"],["FORCE_SETTING","Force setting, replaces any other file or directory"],["IGNORE_MISSING_TARGET_SETTING","Create the symlink even if the target file does not exist"],["PATH_SETTING","Path setting (path of the symlink)"],["RELATIVE_SETTING","TODO: Allow relative symlinks, if false any relative symlinks cause a failure."],["RELINK_SETTING","Relink setting, if true the action relinks an existing symlink (applies if force is false)"],["RESOLVE_SYMLINK_TARGET_SETTING","Resolves the target if it is a symlink and uses the final target file as the target."],["TARGET_SETTING","Target setting (path to the file the symlink points to)"]],"fn":[["init_directive_data","Initialize the defaults for the LinkDirective."]],"struct":[["LinkDirective","A directive that can build [LinkAction]s to create directories in the filesystem."]],"type":[["FakeLinkDirective","[LinkDirective] that uses the native [FakeFileSystem] for testing."],["NativeLinkDirective","[LinkDirective] that uses the native [OsFileSystem]."]]};