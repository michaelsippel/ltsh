

    pub enum AnnotationContext {
        Cached( Vec<(CommandPattern, CommandTypeStatement)> ),
        Load( String ),
        FindIn( String ),
    }
    impl AnnotationContext {
        /* loads & parses any given context
         */
        /*
        pub fn into_cached(self) -> AnnotationContext {
            match self {
                AnnotationContext::Load( path ) => {
                    
                }
            }
        }
*/
        pub fn get_type(&self, cmd: &Command) -> Result<CommandType, UnificationError> {
            match cmd {
                Command::Simple{ assignments, command_word, redirections } => {
                    match self {
                        AnnotationContext::Cached( annotations ) => {
                            // find matching command pattern...
                            for (cmd_pat, typ) in annotations.iter() {
                                if let Ok(unificator) = cmd_pat.match_cmd(cmd) {
                                    return Ok( typ.substitute(unificator).eval() );
                                }
                            }

                            Err(UnificationError::NoPattern)
                        },

                        AnnotationContext::Load( path ) => {
                            /* todo:
                             *   - open file at `path`
                             *   - parse CommandPattern + CommandTypeStatement
                             *   - get_type on AnnotationContext::Cached()
                             */
                            
                        }
                        AnnotationContext::FindIn( path ) => {
    //                    if let Some(command_name) = command_word.segments.get(0) {
                            /* todo:
                             * - use command_name to lookup file
                             * - forward to AnnotationContext::Load()
                             */
/*
                            let mut err = UnificationError( vec![] );
                            for file in path.direntries {
                                if let Ok(typ) = AnnotationContext::Load( path ).get_type() => {
                                    
                                }
                            }
*/
    //                    }
                        }
                    }
                }

                _ => {
                    Err(UnificationError::NoPattern)
                }
            }
        }
    }

