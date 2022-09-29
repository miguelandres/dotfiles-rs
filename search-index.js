var searchIndex = JSON.parse('{\
"dotfiles":{"doc":"","t":[0,5,5,3,3,12,11,11,11,11,11,11,11,12,12,12,11,11,11,12,11,12,12,11,11,11,11,11,11],"n":["flags","main","process","FlagData","FlagParser","all_flag_names","borrow","borrow","borrow_mut","borrow_mut","check_flags_are_valid","from","from","homebrew_install_flag_names","install_homebrew","install_ohmyzsh","into","into","new","ohmyzsh_install_flag_names","parse_flags","skip_chsh","skip_chsh_flag_names","try_from","try_from","try_into","try_into","type_id","type_id"],"q":["dotfiles","","","dotfiles::flags","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","",""],"i":[0,0,0,0,0,3,4,3,4,3,3,4,3,3,4,4,4,3,3,3,3,4,3,4,3,4,3,4,3],"f":[0,[[]],[[],[[2,[1]]]],0,0,0,[[]],[[]],[[]],[[]],[3,[[2,[1]]]],[[]],[[]],0,0,0,[[]],[[]],[[],3],0,[3,[[2,[4,1]]]],0,0,[[],2],[[],2],[[],2],[[],2],[[],5],[[],5]],"p":[[3,"DotfilesError"],[4,"Result"],[3,"FlagParser"],[3,"FlagData"],[3,"TypeId"]]},\
"dotfiles_actions":{"doc":"This crate contains all concrete actions and directives. …","t":[0,0,0,0,0,0,0,0,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,3,17,17,11,11,11,11,11,5,11,11,11,5,11,11,11,11,11,0,0,3,11,11,11,11,11,11,11,11,11,11,11,11,11,3,17,17,17,11,11,11,11,11,11,5,11,11,11,5,11,11,11,11,0,0,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,17,17,17,17,3,11,11,11,11,11,5,11,11,11,5,11,11,11,11,0,3,11,11,11,11,11,11,11,11,11,11,11,0,0,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,17,17,17,17,3,17,17,17,17,17,11,11,11,11,11,11,5,11,11,11,5,11,11,11,11,11,0,3,11,11,11,11,11,11,11,11,11,11,11],"n":["brew","create","exec","homebrew_install","link","ohmyzsh_install","action","directive","BrewAction","borrow","borrow_mut","casks","eq","execute","fmt","force_casks","formulae","from","into","new","taps","try_from","try_into","type_id","BrewDirective","DIRECTIVE_NAME","FORCE_CASKS_SETTING","borrow","borrow_mut","build_action_list","directive_data","from","init_directive_data","into","name","new","new_brew_directive","parse_action","parse_action_list","try_from","try_into","type_id","action","directive","CreateAction","borrow","borrow_mut","directory","eq","execute","fmt","force","from","into","new","try_from","try_into","type_id","CreateDirective","DIRECTIVE_NAME","DIR_SETTING","FORCE_SETTING","borrow","borrow_mut","build_action_list","directive_data","from","fs","init_directive_data","into","name","new","new_native_create_directive","parse_action","try_from","try_into","type_id","action","directive","ExecAction","borrow","borrow_mut","command","description","echo","eq","execute","fmt","from","into","new","try_from","try_into","type_id","COMMAND_SETTING","DESCRIPTION_SETTING","DIRECTIVE_NAME","ECHO_SETTING","ExecDirective","borrow","borrow_mut","build_action_list","directive_data","from","init_directive_data","into","name","new","new_exec_directive","parse_action","try_from","try_into","type_id","action","HomebrewInstallAction","borrow","borrow_mut","check_brew_is_installed","default","execute","from","into","new","try_from","try_into","type_id","action","directive","LinkAction","borrow","borrow_mut","create_parent_dirs","eq","execute","fmt","force","from","ignore_missing_target","into","new","path","relative","relink","resolve_symlink_target","target","try_from","try_into","type_id","CREATE_PARENT_DIRS_SETTING","DIRECTIVE_NAME","FORCE_SETTING","IGNORE_MISSING_TARGET_SETTING","LinkDirective","PATH_SETTING","RELATIVE_SETTING","RELINK_SETTING","RESOLVE_SYMLINK_TARGET_SETTING","TARGET_SETTING","borrow","borrow_mut","build_action_list","directive_data","from","fs","init_directive_data","into","name","new","new_native_link_directive","parse_action","parse_shortened_action","try_from","try_into","type_id","action","OhMyZshInstallAction","borrow","borrow_mut","check_oh_my_zsh_is_installed","check_zsh_is_installed","execute","from","into","new","try_from","try_into","type_id"],"q":["dotfiles_actions","","","","","","dotfiles_actions::brew","","dotfiles_actions::brew::action","","","","","","","","","","","","","","","","dotfiles_actions::brew::directive","","","","","","","","","","","","","","","","","","dotfiles_actions::create","","dotfiles_actions::create::action","","","","","","","","","","","","","","dotfiles_actions::create::directive","","","","","","","","","","","","","","","","","","","dotfiles_actions::exec","","dotfiles_actions::exec::action","","","","","","","","","","","","","","","dotfiles_actions::exec::directive","","","","","","","","","","","","","","","","","","","dotfiles_actions::homebrew_install","dotfiles_actions::homebrew_install::action","","","","","","","","","","","","dotfiles_actions::link","","dotfiles_actions::link::action","","","","","","","","","","","","","","","","","","","","dotfiles_actions::link::directive","","","","","","","","","","","","","","","","","","","","","","","","","","dotfiles_actions::ohmyzsh_install","dotfiles_actions::ohmyzsh_install::action","","","","","","","","","","",""],"d":["This module contains the BrewAction and BrewDirective","This module contains the CreateAction and CreateDirective …","This module contains the ExecAction and ExecDirective used …","This module contains the HomebrewInstallAction","This module contains the LinkAction and LinkDirective …","This module contains the OhMyZshInstallAction","This module contains the BrewAction that installs a brew …","This module defines BrewDirective.","BrewAction Installs software using homebrew.","","","List of casks to install.","","","","Whether to pass <code>--force</code> to cask installation.","List of formulae to install.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Constructs a new BrewAction","List of taps to tap into.","","","","A directive that can build BrewActions to install …","Name of the Brew directive","force casks to deal with previously installed apps","","","","","Returns the argument unchanged.","Initialize the defaults for the BrewDirective.","Calls <code>U::from(self)</code>.","","Create a new BrewDirective","Create a new brew directive.","","Parse the list of actions from yaml, in this case it’s …","","","","This module contains the CreateAction that creates a new …","This module defines CreateDirective.","CreateAction creates a new directory when executed","","","Returns the directory to create.","","Creates the <code>directory</code>.","","Returns true if the action will create parent directories …","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Constructs a new instance of CreateAction","","","","A directive that can build CreateActions to create …","Constant for the name of the <code>create</code> directive.","Constant for the name of the <code>directory</code> Setting which …","Constant for the name of the <code>force</code> Setting which forces to …","","","","","Returns the argument unchanged.","Returns the FileSystem instance being used.","Initializes the default configuration for the …","Calls <code>U::from(self)</code>.","","Constructs a new instance of the create directive.","Constructs a new CreateDirective using the real filesystem","","","","","This module contains the ExecAction that executes a …","This module defines ExecDirective which represents …","ExecAction Installs software using homebrew.","","","The command to run","Description for the command to run.","Whether to print out the command for clarity.","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Create a new Exec Action","","","","Command to run","Optional description for the command to run","Name of the Exec directive","Echo the command to run before running it.","A directive that can build ExecActions to run commands","","","","","Returns the argument unchanged.","Initialize the defaults for the BrewDirective.","Calls <code>U::from(self)</code>.","","Creates a new Exec Directives","Create a new brew directive.","","","","","This module contains the HomebrewInstallAction that …","HomebrewInstallAction installs homebrew.","","","Returns true if it can find a brew command","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Constructs a new HomebrewInstallAction","","","","This module contains the LinkAction that creates a new …","This module defines LinkDirective.","LinkAction creates a new symlink <code>path</code> that points to <code>target</code>…","","","Create all parent directories if they do not exist already","","","","Force to replace an existing file or directory when …","Returns the argument unchanged.","Succeed even if <code>target</code> doesn’t point to an existing file …","Calls <code>U::from(self)</code>.","Constructs a new LinkAction","Path of the new symlink","Allow relative linking. TODO: actually implement relative …","Force to re-create the symlink if it exists already","If the target is another symlink, resolve the ultimate …","Path that the symlink points to.","","","","Create parent dirs if they don’t exist","Name of the link directive","Force setting, replaces any other file or directory","Create the symlink even if the target file does not exist","A directive that can build LinkActions to create …","Path setting (path of the symlink)","TODO: Allow relative symlinks, if false any relative …","Relink setting, if true the action relinks an existing …","Resolves the target if it is a symlink and uses the final …","Target setting (path to the file the symlink points to)","","","","","Returns the argument unchanged.","Returns the FileSystem instance being used.","Initialize the defaults for the LinkDirective.","Calls <code>U::from(self)</code>.","","Create a new LinkDirective","Create a new link directive using the native filesystem","","Parse a shortened action with only link name to target name","","","","This module contains the OhMyZshInstallAction that sets up …","OhMyZshInstallAction sets up ohmyzsh.","","","Returns true if the $ZSH environment var is set","Returns true if it can find a brew command","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Constructs a new OhMyZshInstallAction","","",""],"i":[0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,10,10,10,10,10,0,10,10,10,0,10,10,10,10,10,0,0,0,18,18,18,18,18,18,18,18,18,18,18,18,18,0,0,0,0,19,19,19,19,19,19,0,19,19,19,0,19,19,19,19,0,0,0,22,22,22,22,22,22,22,22,22,22,22,22,22,22,0,0,0,0,0,24,24,24,24,24,0,24,24,24,0,24,24,24,24,0,0,25,25,25,25,25,25,25,25,25,25,25,0,0,0,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,0,0,0,0,0,0,0,0,0,0,28,28,28,28,28,28,0,28,28,28,0,28,28,28,28,28,0,0,29,29,29,29,29,29,29,29,29,29,29],"f":[0,0,0,0,0,0,0,0,0,[[]],[[]],[1],[[1,1],2],[1,[[4,[3]]]],[[1,5],6],[1,2],[1],[[]],[[]],[[2,[8,[7]],[8,[7]],[8,[7]]],1],[1],[[],4],[[],4],[[],9],0,0,0,[[]],[[]],[[10,11,12],[[4,[[8,[[14,[13]]]],3]]]],[10,15],[[]],[[],15],[[]],[10,16],[[],10],[[],10],[[10,11,12],[[4,[1,3]]]],[[10,11,12],[[4,[[8,[1]],3]]]],[[],4],[[],4],[[],9],0,0,0,[[]],[[]],[[[18,[17]]],16],[[[18,[17]],[18,[17]]],2],[[[18,[17]]],[[4,[3]]]],[[[18,[17]],5],6],[[[18,[17]]],2],[[]],[[]],[[7,2],[[18,[17]]]],[[],4],[[],4],[[],9],0,0,0,0,[[]],[[]],[[[19,[17]],11,12],[[4,[[8,[[14,[13]]]],3]]]],[[[19,[17]]],15],[[]],[[[19,[17]]]],[[],15],[[]],[[[19,[17]]],16],[17,[[19,[17]]]],[[],[[19,[20]]]],[[[19,[17]],21,12],[[4,[[18,[17]],3]]]],[[],4],[[],4],[[],9],0,0,0,[[]],[[]],[22,16],[22,[[23,[7]]]],[22,2],[[22,22],2],[22,[[4,[3]]]],[[22,5],6],[[]],[[]],[[7,[23,[7]],2],22],[[],4],[[],4],[[],9],0,0,0,0,0,[[]],[[]],[[24,11,12],[[4,[[8,[[14,[13]]]],3]]]],[24,15],[[]],[[],15],[[]],[24,16],[[],24],[[],24],[[24,21,12],[[4,[22,3]]]],[[],4],[[],4],[[],9],0,0,[[]],[[]],[25,2],[[],25],[25,[[4,[3]]]],[[]],[[]],[[],25],[[],4],[[],4],[[],9],0,0,0,[[]],[[]],[[[27,[[0,[17,26]]]]],2],[[[27,[[0,[17,26]]]],[27,[[0,[17,26]]]]],2],[[[27,[[0,[17,26]]]]],[[4,[3]]]],[[[27,[[0,[17,26]]]],5],6],[[[27,[[0,[17,26]]]]],2],[[]],[[[27,[[0,[17,26]]]]],2],[[]],[[7,7,11,11],[[27,[[0,[17,26]]]]]],[[[27,[[0,[17,26]]]]],7],[[[27,[[0,[17,26]]]]],2],[[[27,[[0,[17,26]]]]],2],[[[27,[[0,[17,26]]]]],2],[[[27,[[0,[17,26]]]]],7],[[],4],[[],4],[[],9],0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[[28,[[0,[17,26]]]],11,12],[[4,[[8,[[14,[13]]]],3]]]],[[[28,[[0,[17,26]]]]],15],[[]],[[[28,[[0,[17,26]]]]]],[[],15],[[]],[[[28,[[0,[17,26]]]]],16],[[[0,[17,26]]],[[28,[[0,[17,26]]]]]],[[],[[28,[20]]]],[[[28,[[0,[17,26]]]],11,12],[[4,[[27,[[0,[17,26]]]],3]]]],[[[28,[[0,[17,26]]]],11,12],[[4,[[27,[[0,[17,26]]]],3]]]],[[],4],[[],4],[[],9],0,0,[[]],[[]],[29,2],[29,2],[29,[[4,[3]]]],[[]],[[]],[2,29],[[],4],[[],4],[[],9]],"p":[[3,"BrewAction"],[15,"bool"],[3,"DotfilesError"],[4,"Result"],[3,"Formatter"],[6,"Result"],[3,"String"],[3,"Vec"],[3,"TypeId"],[3,"BrewDirective"],[6,"Settings"],[4,"Yaml"],[8,"Action"],[3,"Box"],[3,"DirectiveData"],[15,"str"],[8,"FileSystem"],[3,"CreateAction"],[3,"CreateDirective"],[3,"OsFileSystem"],[3,"HashMap"],[3,"ExecAction"],[4,"Option"],[3,"ExecDirective"],[3,"HomebrewInstallAction"],[8,"UnixFileSystem"],[3,"LinkAction"],[3,"LinkDirective"],[3,"OhMyZshInstallAction"]]},\
"dotfiles_core":{"doc":"The core of Dotfiles-rs is basically a set of directives …","t":[2,2,2,2,0,0,0,0,0,0,8,8,16,10,11,10,11,8,3,3,8,11,11,11,11,11,10,11,11,11,11,11,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,13,3,4,13,13,13,13,13,13,11,11,11,11,11,5,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,5,5,13,13,4,6,13,11,11,11,11,11,11,5,11,11,11,11,11,12,12,12,5,5,5,5,5,5,5,5,5],"n":["Action","Directive","Setting","Settings","action","directive","error","exec_wrapper","settings","yaml_util","Action","ActionParser","ActionType","execute","name","parse_action","parse_action_list","Directive","DirectiveData","DirectiveSet","HasDirectiveData","add","borrow","borrow","borrow_mut","borrow_mut","build_action_list","clone","clone_into","default","defaults","defaults","directive_data","fmt","from","from","from","into","into","name","name","new","to_owned","try_from","try_from","try_into","try_into","type_id","type_id","CoreError","DotfilesError","ErrorType","ExecutionError","FileSystemError","IncompleteConfigurationError","TestingErrorActionSucceedsWhenItShouldFail","UnexpectedYamlTypeError","YamlParseError","borrow","borrow","borrow_mut","borrow_mut","error_type","execution_error","fmt","fmt","from","from","from","from_io_error","into","into","message","try_from","try_from","try_into","try_into","type_id","type_id","exit_status","fs_error","missing_field","popen_error","execute_command","execute_pipeline","Boolean","Integer","Setting","Settings","String","borrow","borrow_mut","clone","clone_into","fmt","from","initialize_settings_object","into","to_owned","try_from","try_into","type_id","0","0","0","get_boolean_setting","get_boolean_setting_from_yaml_or_defaults","get_int_setting","get_integer_setting_from_yaml_or_defaults","get_setting","get_string_array","get_string_content_or_keyed_value","get_string_setting","get_string_setting_from_yaml_or_defaults"],"q":["dotfiles_core","","","","","","","","","","dotfiles_core::action","","","","","","","dotfiles_core::directive","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","dotfiles_core::error","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","dotfiles_core::error::ErrorType","","","","dotfiles_core::exec_wrapper","","dotfiles_core::settings","","","","","","","","","","","","","","","","","dotfiles_core::settings::Setting","","","dotfiles_core::yaml_util","","","","","","","",""],"d":["","","","","This module contains the base trait for all Actions.","This module contains the base trait for all Directive and …","Module for the error handling classes and enums.","Wraps some logic to run external commands and handle errors","This module contains the definition of a setting and code …","Module that defines helper functions to process YAML …","An action to be run by a the dotfiles runtime.","Trait to parse a specific action type from Yaml.","The action type this object parses","Executes the action.","The name of the action this object parses","Builds a single action of type ActionParser::ActionType …","Builds a list of actions of type ActionParser::ActionType …","A parser for action steps, each directive represents a …","A struct that contains the default settings for a …","A struct that contains the currently registered directives.","A trait for all directives, it is shared between …","Add a new directive","","","","","Builds a list of actions for this directive from a Yaml …","","","","Returns the default settings as configured.","Default settings for this directive.","Returns the directive data for this object","","Returns the argument unchanged.","Constructs a new directive from a name and a set of …","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Returns the name of the directive.","Unique name of this directive.","Create a new directive set","","","","","","","","A core logic error for Dotfiles-rs","Struct that represents an error that happened while …","A collection of types of errors that may occur while …","An error occurred while running a command necessary for …","A filesystem error that was encountered while either …","The configuration is missing a required field","An error only for testing, the action that should fail …","Received an Yaml object of an unexpected type","An error that occurred while parsing the Yaml file","","","","","Error type","Creates an ErrorType::ExecutionError","","","Returns the argument unchanged.","Creates a new Dotfiles error with the given message and …","Returns the argument unchanged.","Creates a new Dotfiles error with the given message and …","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Human-readable error message","","","","","","","If the command attempted to execute but failed for some …","The underlying filesystem error.","Name of the field missing in the configuration","If the command could not execute for some reason the …","Executes the <code>cmd</code> and waits for it to finish.","Executes the <code>pipeline</code> and waits for it to finish.","A boolean value for a setting","An Integer value for a setting","Represents a value for a setting","The Settings object is a hashmap of option names to a …","A string value for a setting","","","","","","Returns the argument unchanged.","Returns a Settings object from an array as a bit of …","Calls <code>U::from(self)</code>.","","","","","","","","Gets a boolean value for the setting named <code>name</code>.","Gets a Boolean value from YAML or defaults.","Gets a Int value for the setting named <code>name</code>.","Gets a Integer value from YAML or defaults.","Gets a String value for the setting named <code>name</code>.","Gets a native <code>Vec&lt;String&gt;</code> from a Yaml::Array. It errors …","Gets the content of this YAML node or the value for a …","Gets a String value for the setting named <code>name</code>.","Gets a String value from YAML or defaults."],"i":[0,0,0,0,0,0,0,0,0,0,0,0,26,10,26,26,26,0,0,0,0,7,7,11,7,11,8,11,11,7,8,11,27,11,7,11,11,7,11,8,11,7,11,7,11,7,11,7,11,16,0,0,16,16,16,16,16,16,16,1,16,1,1,0,16,1,16,1,1,1,16,1,1,16,1,16,1,16,1,28,29,30,28,0,0,23,23,0,0,23,23,23,23,23,23,23,0,23,23,23,23,23,31,32,33,0,0,0,0,0,0,0,0,0],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,[[],[[2,[1]]]],[[],3],[[4,5],[[2,[1]]]],[[4,5],[[2,[6,1]]]],0,0,0,0,[[7,3,[9,[8]]],[[2,[1]]]],[[]],[[]],[[]],[[]],[[4,5],[[2,[[6,[[9,[10]]]],1]]]],[11,11],[[]],[[],7],[[],4],[11,4],[[],11],[[11,12],13],[[]],[[14,4],11],[[]],[[]],[[]],[[],3],[11,14],[[],7],[[]],[[],2],[[],2],[[],2],[[],2],[[],15],[[],15],0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[1,16],[[[18,[17]],[18,[19]]],16],[[16,12],13],[[1,12],13],[[]],[[14,16],1],[[]],[20,1],[[]],[[]],[1,14],[[],2],[[],2],[[],2],[[],2],[[],15],[[],15],0,0,0,0,[[21,3,3],[[2,[1]]]],[[22,3,3],[[2,[1]]]],0,0,0,0,0,[[]],[[]],[23,23],[[]],[[23,12],13],[[]],[[],4],[[]],[[]],[[],2],[[],2],[[],15],0,0,0,[[3,4,4],[[2,[24,1]]]],[[3,5,4,4],[[2,[24,1]]]],[[3,4,4],[[2,[25,1]]]],[[3,5,4,4],[[2,[25,1]]]],[[3,4,4],[[2,[23,1]]]],[[5,3],[[2,[[6,[14]],1]]]],[[5,[18,[3]]],[[2,[14,1]]]],[[3,4,4],[[2,[14,1]]]],[[3,5,4,4],[[2,[14,1]]]]],"p":[[3,"DotfilesError"],[4,"Result"],[15,"str"],[6,"Settings"],[4,"Yaml"],[3,"Vec"],[3,"DirectiveSet"],[8,"Directive"],[3,"Box"],[8,"Action"],[3,"DirectiveData"],[3,"Formatter"],[6,"Result"],[3,"String"],[3,"TypeId"],[4,"ErrorType"],[4,"PopenError"],[4,"Option"],[4,"ExitStatus"],[3,"Error"],[3,"Exec"],[3,"Pipeline"],[4,"Setting"],[15,"bool"],[15,"i64"],[8,"ActionParser"],[8,"HasDirectiveData"],[13,"ExecutionError"],[13,"FileSystemError"],[13,"IncompleteConfigurationError"],[13,"Boolean"],[13,"String"],[13,"Integer"]]},\
"dotfiles_core_macros":{"doc":"This crate provides procedural macros to generate …","t":[24],"n":["ActionListDirective"],"q":["dotfiles_core_macros"],"d":["Generates a Directive&lt;’a&gt; implementation for the struct …"],"i":[0],"f":[0],"p":[]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
