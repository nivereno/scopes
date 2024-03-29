use core::time;
use std::{fmt, collections::hash_map::DefaultHasher, hash::{Hash, Hasher}, thread};
use bimap::BiMap;

pub struct SymbolMap {
    pub map: BiMap<String, Symbol>,
    pub num_symbols :u64 
}

impl SymbolMap {
    pub fn get_mapped_symbol_name(&mut self, id: &Symbol) -> Option<&String> {
        let name = self.map.get_by_right(id);
        return name
    }

    fn map_symbol(&mut self, id: Symbol, name: String) {
        self.map.insert(name, id);
    }

    fn verify_unmapped(&self, id: &Symbol, name: &String) -> Option<i32> {
        let mut mapped = 0;
        if let symbol = self.map.get_by_left(name) {
            print!("symbol: {} is already mapped to {}", symbol.unwrap(), name);
            mapped+=1;
        }
        if let result_name = self.map.get_by_right(id) {
            print!("name: {} is already mapped to {}", result_name.unwrap(), id);
            mapped+=1;
        }
        if mapped == 0 {
            return None;
        }
        return Some(mapped);
    }

    fn map_known_symbol(&mut self, id: Symbol, name: String) {
        if let already_mapped = self.verify_unmapped(&id, &name) {
            print!("symbol or id already mapped");
            return;
        }
        self.map_symbol(id, name);
    }
    
    pub fn add_symbol(&mut self, name: String) -> Symbol { //Previously get_symbol.... for some reason
        if let Some(sym) = self.map.get_by_left(&name) {
            if let Some(oldname) = self.map.get_by_right(&sym) {
                if oldname != &name {
                    print!("internal error: symbol hash collision between {} and {}", name, name); //not quite right
                }
            }
        }
        self.num_symbols += 1;
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        let id = hasher.finish();
        self.map_symbol(Symbol(id), name);
        return Symbol(id);
    }

    #[deprecated(note="please use 'add_symbol' instead")]
    #[allow(dead_code)]
    pub fn get_symbol(_name: String) { unimplemented!() }
}




#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Symbol(pub u64);


impl Symbol {
    pub fn init_symbols(symbols: &mut BiMap<String, Symbol>) {
        let SCOPES_SYMBOLS = vec![(String::from("TIMER_Main"), Symbol(KnownSymbol::TIMER_Main as u64)), (String::from(""), Symbol(KnownSymbol::SYM_Unnamed as u64)), (String::from("?corrupted?"), Symbol(KnownSymbol::SYM_Corrupted as u64)), (String::from("fn"), Symbol(KnownSymbol::KW_Fn as u64))];
        symbols.extend(SCOPES_SYMBOLS.into_iter());
        thread::sleep(time::Duration::from_millis(100)); //Testing
    }
    pub fn value(&self) -> u64 {
        todo!()
    }
    //Maybe constructors but maybe unnecessary
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Symbol(i) = self;
        return write!(f, "{}", i)
    }
}

enum SymbolTypeEnum {
    B_SPIRV_STORAGE_CLASS,
    B_SPIRV_DIM,
    B_SPIRV_IMAGE_FORMAT,
    B_SPIRV_BUILTINS,
    B_SPIRV_EXECUTION_MODE,
    B_SPIRV_IMAGE_OPERAND,
    SCOPES_BUILTIN_SUGAR_SYMBOLS,
    SCOPES_BUILTIN_SPICE_SYMBOLS,
    SCOPES_BUILTIN_SYMBOLS,
    SCOPES_LIBRARY_SUGAR_SYMBOLS,
    SCOPES_LIBRARY_SPICE_SYMBOLS,
    SCOPES_LIBRARY_SFXSPICE_SYMBOLS,
    SCOPES_LIBRARY_OPERATOR_SYMBOLS,
    SCOPES_LIBRARY_SYMBOLS,
    SCOPES_LEGACY_SYMBOLS,
    SCOPES_STYLE_SYMBOLS,
    SCOPES_SYMBOLS,
}



#[repr(u64)]
pub enum KnownSymbol {
    SYM_Unnamed = 0, /**/
    SYM_Corrupted = 8293695010788208856, /*?corrupted?*/
    KW_Fn = 11791429237585314705, /*fn*/
    KW_Inline = 7424826876975356284, /*inline*/
    KW_Label = 15598826335259526680, /*label*/
    KW_SyntaxQuote = 13251267634460441998, /*sugar-quote*/
    KW_Forward = 17580625892724158845, /*_*/
    KW_Raise = 10038550122701687198, /*raise*/
    KW_Call = 11830146953634196313, /*call*/
    KW_RawCall = 16420046355870272743, /*rawcall*/
    KW_Do = 16318354259569728807, /*do*/
    KW_DoIn = 11384528900441371607, /*embed*/
    KW_Try = 18425431747464301132, /*try*/
    KW_Return = 14497890071407333186, /*return*/
    KW_Loop = 15196972976532682029, /*loop*/
    KW_Repeat = 11347821811800953042, /*repeat*/
    KW_Break = 17211832722083645122, /*break*/
    KW_Merge = 14957133470269204328, /*merge*/
    KW_ASTQuote = 7430679618541845939, /*spice-quote*/
    KW_ASTUnquote = 7350642034361413565, /*spice-unquote*/
    KW_ASTUnquoteArguments = 16998221111552564562, /*spice-unquote-arguments*/
    KW_Let = 5790167935852426181, /*let*/
    KW_IndirectLet = 10666060143242234665, /*indirect-let*/
    KW_If = 13371470842486420833, /*if*/
    KW_Switch = 16347363301748809842, /*switch*/
    FN_GetSyntaxScope = 15416161862801919619, /*__this-scope*/
    KW_RunStage = 17794376515602042990, /*run-stage*/
    KW_SyntaxLog = 13415752154434022632, /*sugar-log*/
    KW_Using = 18307978836815453780, /*using*/
    FN_Returning = 11200528088251475402, /*returning*/
    FN_Raising = 3003877888974392369, /*raising*/
    FN_Branch = 6054936783124503815, /*branch*/
    FN_Dump = 13629055916778631757, /*dump*/
    FN_DumpTemplate = 421321401726179009, /*dump-template*/
    FN_DumpDebug = 5463584281655217479, /*dump-debug*/
    FN_DumpAST = 10996714358628325252, /*dump-spice*/
    FN_DumpUniques = 7316208953821230996, /*dump-uniques*/
    FN_Alloca = 7698786966567604067, /*alloca*/
    FN_Malloc = 14496291330061790866, /*malloc*/
    FN_Free = 13262137569111239226, /*free*/
    FN_AllocaArray = 1313331349477796196, /*alloca-array*/
    FN_MallocArray = 9244359448953440738, /*malloc-array*/
    FN_Dupe = 16928516792864583331, /*dupe*/
    FN_Move = 13111939806849960724, /*move*/
    FN_View = 10657230451770015536, /*view*/
    FN_Viewing = 13878673197005213425, /*viewing*/
    FN_Lose = 14240713189781333007, /*lose*/
    FN_IsDropped = 6136202850004329580, /*dropped?*/
    FN_Assign = 14841666659435785157, /*assign*/
    FN_Deref = 12990872281951919278, /*deref*/
    FN_PtrToRef = 6896045144306063821, /*ptrtoref*/
    FN_RefToPtr = 826050025546261424, /*reftoptr*/
    FN_HideTraceback = 16762081803009273005, /*hide-traceback*/
    FN_IsValid = 8732334173936826724, /*unique-visible?*/
    OP_ICmpEQ = 5206430646276843146, /*icmp==*/
    OP_ICmpNE = 205028554998644533, /*icmp!=*/
    FN_Sample = 7146986931670461388, /*sample*/
    FN_ImageRead = 1335622778316589473, /*Image-read*/
    FN_ImageWrite = 3993274272426582819, /*Image-write*/
    SYM_DropHandler = 13124638559887598579, /*__drop*/
    FN_ImageQuerySize = 4539788270915132007, /*Image-query-size*/
    FN_ImageQueryLod = 2013337743233464988, /*Image-query-lod*/
    FN_ImageQueryLevels = 4777672724091325282, /*Image-query-levels*/
    FN_ImageQuerySamples = 9881152918598993053, /*Image-query-samples*/
    FN_ImageTexelPointer = 13495407634310963459, /*Image-texel-pointer*/
    OP_CmpXchg = 2929705335082601657, /*cmpxchg*/
    OP_Barrier = 9251807398228230925, /*__barrier*/
    OP_AtomicRMW = 5065287790760123262, /*atomicrmw*/
    SYM_Atomic = 10059735532003872687, /*atomic*/
    SYM_Volatile = 4203911951653064067, /*volatile*/
    OP_ICmpUGT = 4074119212043174623, /*icmp>u*/
    OP_ICmpUGE = 3647194115691986635, /*icmp>=u*/
    OP_ICmpULT = 14926232692604959802, /*icmp<u*/
    OP_ICmpULE = 2425419303091259192, /*icmp<=u*/
    OP_ICmpSGT = 12504572950124258245, /*icmp>s*/
    OP_ICmpSGE = 16623845847212508113, /*icmp>=s*/
    OP_ICmpSLT = 1398211556837597317, /*icmp<s*/
    OP_ICmpSLE = 760187431645327454, /*icmp<=s*/
    OP_FCmpOEQ = 16218787100603330140, /*fcmp==o*/
    OP_FCmpONE = 16895892424085049336, /*fcmp!=o*/
    OP_FCmpORD = 12330810909539606019, /*fcmp-ord*/
    OP_FCmpOGT = 3718137169719293477, /*fcmp>o*/
    OP_FCmpOGE = 8870308047783970072, /*fcmp>=o*/
    OP_FCmpOLT = 3281281873731931116, /*fcmp<o*/
    OP_FCmpOLE = 3102427266164917098, /*fcmp<=o*/
    OP_FCmpUEQ = 11936173757777724392, /*fcmp==u*/
    OP_FCmpUNE = 7226058624861464143, /*fcmp!=u*/
    OP_FCmpUNO = 12673792879206270398, /*fcmp-uno*/
    OP_FCmpUGT = 18386238483783145311, /*fcmp>u*/
    OP_FCmpUGE = 6338058335004206001, /*fcmp>=u*/
    OP_FCmpULT = 14688547897545054789, /*fcmp<u*/
    OP_FCmpULE = 3789387272942834734, /*fcmp<=u*/
    FN_TypeOf = 1902888756976800627, /*typeof*/
    FN_Bitcast = 11979030388760670942, /*bitcast*/
    FN_IntToPtr = 339192923846162963, /*inttoptr*/
    FN_PtrToInt = 8079619589193629855, /*ptrtoint*/
    FN_Load = 7294993249745808601, /*load*/
    FN_Store = 7302668838912293785, /*store*/
    FN_VolatileLoad = 4850046681307874964, /*volatile-load*/
    FN_VolatileStore = 17043446401604594808, /*volatile-store*/
    SFXFN_ExecutionMode = 14107133208799409460, /*set-execution-mode*/
    FN_ExtractElement = 18069182659200308872, /*extractelement*/
    FN_InsertElement = 6535415653728777853, /*insertelement*/
    FN_ShuffleVector = 846550015229781369, /*shufflevector*/
    FN_ExtractValue = 12983393784012802349, /*extractvalue*/
    FN_InsertValue = 3530704265121648383, /*insertvalue*/
    FN_SwapValue = 10822852771475010717, /*swapvalue*/
    FN_ITrunc = 12168583478537439100, /*itrunc*/
    FN_ZExt = 14543552449570244740, /*zext*/
    FN_SExt = 5418482068826672714, /*sext*/
    FN_GetElementRef = 14952006627588444780, /*getelementref*/
    FN_GetElementPtr = 6344588776665473435, /*getelementptr*/
    FN_OffsetOf = 16902598683186172609, /*offsetof*/
    FN_VaCountOf = 13505081888065503334, /*va-countof*/
    FN_Undef = 15814674941360812603, /*undef*/
    FN_NOf = 711648085469838905, /*nof*/
    SFXFN_Discard = 432996000833769480, /*discard*/
    SFXFN_Unreachable = 10247097095672061929, /*unreachable*/
    FN_FPTrunc = 3423230818242098309, /*fptrunc*/
    FN_FPExt = 7935562471022698986, /*fpext*/
    FN_FPToUI = 2619874347917667986, /*fptoui*/
    FN_FPToSI = 13673082371534614147, /*fptosi*/
    FN_UIToFP = 11226303482915351744, /*uitofp*/
    FN_SIToFP = 3140091286791132202, /*sitofp*/
    OP_Add = 1960740420766515403, /*add*/
    OP_AddNUW = 152275781108810972, /*add-nuw*/
    OP_AddNSW = 15907393985378643679, /*add-nsw*/
    OP_Sub = 8254688374789905860, /*sub*/
    OP_SubNUW = 15803134504223999187, /*sub-nuw*/
    OP_SubNSW = 17732918368427564510, /*sub-nsw*/
    OP_Mul = 7349689347911243494, /*mul*/
    OP_MulNUW = 9685190023102014764, /*mul-nuw*/
    OP_MulNSW = 13408468746019446411, /*mul-nsw*/
    OP_SDiv = 9181717700611350360, /*sdiv*/
    OP_UDiv = 2115518015486054720, /*udiv*/
    OP_SRem = 3660802082956507259, /*srem*/
    OP_URem = 1148859154261294275, /*urem*/
    OP_Shl = 10295766745010537456, /*shl*/
    OP_LShr = 15470122985773147594, /*lshr*/
    OP_AShr = 2813706706601003049, /*ashr*/
    OP_BAnd = 5211203396938986985, /*band*/
    OP_BOr = 16597581972852507787, /*bor*/
    OP_BXor = 9495087005692069792, /*bxor*/
    OP_BitReverse = 16798685626671407905, /*bitreverse*/
    OP_BitCount = 5487576127299987477, /*bitcount*/
    OP_FindMSB = 8655170277816957870, /*findmsb*/
    OP_FindLSB = 12291529904039426481, /*findlsb*/
    OP_FNeg = 3972828220543378786, /*fneg*/
    OP_FAdd = 17025303705120881131, /*fadd*/
    OP_FSub = 12438664161644387184, /*fsub*/
    OP_FMul = 16337947897024148532, /*fmul*/
    OP_FDiv = 3895221756807556339, /*fdiv*/
    OP_FRem = 882837492719997313, /*frem*/
    OP_BNAnd = 4707329697569727090, /*bnand*/
    OP_SMin = 8607537403102034938, /*smin*/
    OP_SMax = 5262560708027491884, /*smax*/
    OP_UMin = 15785290442074289767, /*umin*/
    OP_UMax = 8854048641347072269, /*umax*/
    OP_Xchg = 12513675509613146379, /*xchg*/
    OP_Tertiary = 6527480048044325118, /*?*/
    OP_FMix = 13286487931641994204, /*fmix*/
    OP_Step = 18426953201814252253, /*step*/
    FN_Round = 2174300061981222401, /*round*/
    FN_RoundEven = 16246837239121262241, /*roundeven*/
    OP_Trunc = 10400484755077226519, /*trunc*/
    OP_FAbs = 7339835430741497992, /*fabs*/
    OP_FSign = 16447520512434442077, /*fsign*/
    OP_SSign = 17434212607490290346, /*ssign*/
    OP_Floor = 13544148793679537295, /*floor*/
    OP_Radians = 9548661528383033479, /*radians*/
    OP_Degrees = 4136927749225470563, /*degrees*/
    OP_Sin = 2007215246322646759, /*sin*/
    OP_Cos = 6067723630743649505, /*cos*/
    OP_Tan = 10130411939769417676, /*tan*/
    OP_Asin = 14975410630834049739, /*asin*/
    OP_Acos = 10927202141045256672, /*acos*/
    OP_Atan = 4905478360142715288, /*atan*/
    OP_Sinh = 7295393251825654477, /*sinh*/
    OP_Cosh = 13899729794716201589, /*cosh*/
    OP_Tanh = 18188592938617184358, /*tanh*/
    OP_ASinh = 15434538529020992897, /*asinh*/
    OP_ACosh = 9082897981064898913, /*acosh*/
    OP_ATanh = 11472678640027442201, /*atanh*/
    OP_Atan2 = 10559460099474921799, /*atan2*/
    OP_Exp = 2049944506156198918, /*exp*/
    OP_Log = 11342747539914778808, /*log*/
    OP_Exp2 = 17737145654003088218, /*exp2*/
    OP_Log2 = 5623879491386029038, /*log2*/
    OP_Pow = 8884534925585743282, /*powf*/
    OP_Sqrt = 17103601657695359693, /*sqrt*/
    OP_InverseSqrt = 9424628734557438287, /*inversesqrt*/
    FN_Fma = 8757314485962196908, /*fma*/
    FN_Frexp = 16686677596629003825, /*frexp*/
    FN_Ldexp = 3478048123506854097, /*ldexp*/
    FN_Length = 17230971456670748934, /*length*/
    FN_Cross = 1583866247426611739, /*cross*/
    FN_Normalize = 14539228102339622825, /*normalize*/
    KW_CatRest = 13965453015604999790, /*::**/
    KW_CatOne = 9056965273203988802, /*::@*/
    KW_Assert = 7434937302410864195, /*assert*/
    KW_Continue = 17151434717616726934, /*continue*/
    KW_Define = 344886856573330070, /*define*/
    KW_DumpSyntax = 5811152217364020920, /*dump-syntax*/
    KW_Then = 688617506041310952, /*then*/
    KW_Else = 2561605952033230217, /*else*/
    KW_ElseIf = 12932461098531191073, /*elseif*/
    KW_EmptyList = 4034324158034789857, /*empty-list*/
    KW_EmptyTuple = 9835153353352501150, /*empty-tuple*/
    KW_Escape = 16455107192035828263, /*escape*/
    KW_Except = 3542135088686406937, /*except*/
    KW_False = 6039321692715317023, /*false*/
    KW_FnTypes = 10514710638501949235, /*fn-types*/
    KW_FnCC = 4187026067042855205, /*fn/cc*/
    KW_Globals = 18030968675306579997, /*globals*/
    KW_In = 13134362065057148930, /*in*/
    KW_LoopFor = 6710767965504681497, /*loop-for*/
    KW_None = 6614300871457773169, /*none*/
    KW_N = 4263125973135525232, /*n*/
    KW_Splice = 1658902670141672343, /*splice*/
    KW_True = 3441749586647789381, /*true*/
    KW_Unquote = 3456458106593020319, /*unquote*/
    KW_UnquoteSplice = 2784147457274486434, /*unquote-splice*/
    KW_ListEmpty = 16169814254371936175, /*eol*/
    KW_With = 1852234971714581248, /*with*/
    KW_XFn = 16478583181152881300, /*xfn*/
    KW_XLet = 18375906608815456046, /*xlet*/
    KW_Yield = 2798692997155704158, /*yield*/
    KW_Recur = 11334795196263018267, /*this-function*/
    FN_VaAt = 1728613234008756115, /*va@*/
    FN_Alignof = 10229826854700409783, /*alignof*/
    FN_Alloc = 6410267048316231547, /*alloc*/
    FN_Arrayof = 6925478141270557499, /*arrayof*/
    FN_AnchorPath = 14623757339740924071, /*Anchor-path*/
    FN_AnchorLineNumber = 3531876622566968915, /*Anchor-line-number*/
    FN_AnchorColumn = 8973823957637956274, /*Anchor-column*/
    FN_AnchorOffset = 10234804462234903284, /*Anchor-offset*/
    FN_AnchorSource = 2361358535910617454, /*Anchor-source*/
    FN_ActiveAnchor = 1994444072710509338, /*active-anchor*/
    FN_ActiveFrame = 12461373209905848162, /*active-frame*/
    FN_BitCountOf = 5688051758412107463, /*bitcountof*/
    FN_IsSigned = 13502642242251275217, /*signed?*/
    FN_BlockMacro = 8106862524427380603, /*block-macro*/
    FN_BlockScopeMacro = 8476260440877030536, /*block-scope-macro*/
    FN_BoolEq = 6247523884735707280, /*bool==*/
    FN_BuiltinEq = 5010317712562993983, /*Builtin==*/
    KW_Case = 4046808499584078618, /*case*/
    KW_Default = 4237700516061554991, /*default*/
    FN_IsCallable = 16032063074414639290, /*callable?*/
    FN_Cast = 16697841417332331018, /*cast*/
    FN_Concat = 18236376666523427191, /*concat*/
    FN_Cons = 1501487700294795692, /*cons*/
    FN_IsConstant = 18013729673973659414, /*constant?*/
    FN_Countof = 11694606181073962112, /*countof*/
    KW_Pass = 9902804511522324737, /*pass*/
    FN_Compile = 6445799659537073858, /*__compile*/
    FN_CompileSPIRV = 613157195900127079, /*__compile-spirv*/
    FN_CompileGLSL = 18396553192831503822, /*__compile-glsl*/
    FN_CompileObject = 12063563248629107173, /*__compile-object*/
    FN_ElementIndex = 405674980502231855, /*element-index*/
    FN_ElementName = 16436677680454778877, /*element-name*/
    FN_Annotate = 12416694325716635040, /*annotate*/
    FN_CStr = 5304973390545619635, /*cstr*/
    FN_DatumToSyntax = 13377532473085950439, /*datum->syntax*/
    FN_DatumToQuotedSyntax = 11910347832707670573, /*datum->quoted-syntax*/
    FN_LabelDocString = 8504600420340326851, /*Label-docstring*/
    FN_LabelSetInline = 4630432271270185876, /*Label-set-inline*/
    FN_DefaultStyler = 6056625628540701078, /*default-styler*/
    FN_StyleToString = 8513446256855055015, /*style->string*/
    FN_Disqualify = 10785811074327527659, /*disqualify*/
    FN_DumpList = 15653434771618198050, /*dump-list*/
    FN_DumpFrame = 13784123353829559607, /*dump-frame*/
    FN_ClosureLabel = 10335615881299646190, /*Closure-label*/
    FN_ClosureFrame = 1172997353190255287, /*Closure-frame*/
    FN_FormatFrame = 5082286120361249323, /*Frame-format*/
    FN_ElementType = 17485435483591664812, /*element-type*/
    FN_IsEmpty = 9057094845911929361, /*empty?*/
    FN_TypeCountOf = 6142431585693719110, /*type-countof*/
    FN_Enumerate = 12738203806750959812, /*enumerate*/
    FN_Eval = 13690284164051263532, /*eval*/
    FN_Exit = 8990025764778293972, /*exit*/
    FN_Expand = 15315018733738648379, /*expand*/
    FN_ExternLibrary = 7319596188986785182, /*extern-library*/
    FN_ExtractMemory = 15333717088260170127, /*extract-memory*/
    FN_EnterSolverCLI = 11486126462863868479, /*enter-solver-cli*/
    FN_FFISymbol = 4478020594069533033, /*ffi-symbol*/
    FN_FFICall = 15900262666103834834, /*ffi-call*/
    FN_FrameEq = 13527292134322085205, /*Frame==*/
    FN_GetExceptionHandler = 6934838721451963672, /*get-exception-handler*/
    FN_GetScopeSymbol = 17074790219283712233, /*get-scope-symbol*/
    FN_Hash = 7855998301446388822, /*__hash*/
    FN_Hash2x64 = 10412393740640633301, /*__hash2x64*/
    FN_HashBytes = 13699556344748761486, /*__hashbytes*/
    FN_RealPath = 17118229553259514997, /*realpath*/
    FN_DirName = 10211206402460697128, /*dirname*/
    FN_BaseName = 18229663198810695345, /*basename*/
    FN_IsFile = 11806041062688905550, /*file?*/
    FN_IsDirectory = 13890673151148027441, /*directory?*/
    FN_ImportC = 9506299690382060405, /*import-c*/
    FN_IsInteger = 6105040647337854385, /*integer?*/
    FN_IntegerType = 2516027684846949869, /*integer-type*/
    FN_CompilerVersion = 8230086638682738611, /*compiler-version*/
    FN_Iter = 15236009938751922191, /*iter*/
    FN_FormatMessage = 17472503434606238878, /*format-message*/
    FN_IsIterator = 15199726907792030222, /*iterator?*/
    FN_IsLabel = 14345247769009550874, /*label?*/
    FN_LabelEq = 1920848156203443267, /*Label==*/
    FN_LabelNew = 834740677155599269, /*Label-new*/
    FN_LabelParameterCount = 10604014272538243396, /*Label-parameter-count*/
    FN_LabelParameter = 1101829193493263067, /*Label-parameter*/
    FN_LabelAnchor = 14968777262082025481, /*Label-anchor*/
    FN_LabelName = 10755043536348143055, /*Label-name*/
    FN_ClosureEq = 5188196795176544534, /*Closure==*/
    FN_CheckStack = 7256069828285740933, /*verify-stack*/
    FN_ListAtom = 14127872484814243206, /*list-atom?*/
    FN_ListCountOf = 12476217828748032377, /*list-countof*/
    FN_ListLoad = 1694270055549364417, /*list-load*/
    FN_ListJoin = 3306358151615545091, /*list-join*/
    FN_ListParse = 2214552977188211461, /*list-parse*/
    FN_IsList = 18069115110623258650, /*list?*/
    FN_LoadLibrary = 14773020576929535446, /*load-library*/
    FN_LabelCountOfReachable = 1202449554590866633, /*Label-countof-reachable*/
    FN_ListAt = 13501294157157117403, /*list-at*/
    FN_ListNext = 5539067424806550949, /*list-next*/
    FN_ListCons = 9540303879138784460, /*list-cons*/
    FN_IsListEmpty = 11607797973743323688, /*list-empty?*/
    FN_Macro = 12237958358133701538, /*macro*/
    FN_Max = 16188263393367025924, /*max*/
    FN_Min = 9213039644321769006, /*min*/
    FN_MemCopy = 14275554572601404045, /*memcopy*/
    FN_IsMutable = 8039807012326762487, /*mutable?*/
    FN_IsNone = 2081831568669205508, /*none?*/
    FN_IsN = 11577081519827334321, /*n?*/
    FN_OrderedBranch = 7367767789571000488, /*ordered-branch*/
    FN_ParameterEq = 822679608152097744, /*Parameter==*/
    FN_ParameterNew = 12335594289940117753, /*Parameter-new*/
    FN_ParameterName = 3438719512471673864, /*Parameter-name*/
    FN_ParameterAnchor = 14703448448964420378, /*Parameter-anchor*/
    FN_ParameterIndex = 3140065852232657490, /*Parameter-index*/
    FN_ParseC = 15632794935753336391, /*parse-c*/
    FN_PointerOf = 4266812011740088774, /*pointerof*/
    FN_PointerType = 3366810928215287158, /*pointer-type*/
    FN_PointerFlags = 1019599642840081884, /*pointer-type-flags*/
    FN_PointerSetFlags = 2329085878009913695, /*pointer-type-set-flags*/
    FN_PointerStorageClass = 4592451964355444539, /*pointer-type-storage-class*/
    FN_PointerSetStorageClass = 9087110657019046953, /*pointer-type-set-storage-class*/
    FN_PointerSetElementType = 12543828769385725254, /*pointer-type-set-element-type*/
    FN_ExternLocation = 8254860855315810262, /*extern-type-location*/
    FN_ExternBinding = 3540962020093752122, /*extern-type-binding*/
    FN_FunctionTypeIsVariadic = 6144624935763156996, /*function-type-variadic?*/
    FN_ArrayType = 9241823069732702838, /*array-type*/
    FN_ImageType = 14320297521175998502, /*Image-type*/
    FN_SampledImageType = 14462055862541679017, /*SampledImage-type*/
    FN_TypenameType = 1576800857155768583, /*typename-type*/
    FN_Write = 4545295222702948510, /*io-write*/
    FN_Flush = 18311511013827746297, /*io-flush*/
    FN_Product = 17562826958642036266, /*product*/
    FN_Prompt = 866754396603858091, /*__prompt*/
    FN_Qualify = 7421500257953923459, /*qualify*/
    FN_SetAutocompleteScope = 10306144975461132892, /*set-autocomplete-scope*/
    FN_Range = 11076469759326625595, /*range*/
    FN_RefNew = 6432373563739579378, /*ref-new*/
    FN_RefAt = 10801776081507347562, /*ref@*/
    FN_Repr = 11282163101405648578, /*Any-repr*/
    FN_AnyString = 645153819934145355, /*Any-string*/
    FN_Require = 15576950620686862063, /*require*/
    FN_ScopeAt = 14972861385411075110, /*Scope@*/
    FN_ScopeLocalAt = 15795284693121517841, /*Scope-local@*/
    FN_ScopeEq = 4307292760091962388, /*Scope==*/
    FN_ScopeNew = 6736439071477289334, /*Scope-new*/
    FN_ScopeCopy = 8829687521009154415, /*Scope-clone*/
    FN_ScopeDocString = 4229124540716802758, /*Scope-docstring*/
    FN_SetScopeDocString = 16302012360655073094, /*set-scope-docstring*/
    FN_ScopeNewSubscope = 9732599281094011015, /*Scope-new-expand*/
    FN_ScopeCopySubscope = 14750733959174261744, /*Scope-clone-expand*/
    FN_ScopeParent = 1215711284137026275, /*Scope-parent*/
    FN_ScopeNext = 13464042205376655131, /*Scope-next*/
    FN_SizeOf = 16571385887714733399, /*sizeof*/
    FN_TypeNext = 10217510036774289743, /*type-next*/
    FN_Slice = 8041556399485751409, /*slice*/
    FN_StringAt = 515762842845798094, /*string@*/
    FN_StringCmp = 1944803912210276633, /*string-compare*/
    FN_StringCountOf = 16137119188615654517, /*string-countof*/
    FN_StringNew = 10434713519260937313, /*string-new*/
    FN_StringJoin = 7318783391468166949, /*string-join*/
    FN_StringSlice = 13453761129941151230, /*string-slice*/
    FN_StructOf = 5258656587470789565, /*structof*/
    FN_TypeStorage = 362384507592211026, /*storageof*/
    FN_IsOpaque = 10931061529502177575, /*opaque?*/
    FN_SymbolEq = 7066406266214372911, /*Symbol==*/
    FN_SymbolNew = 12662486499858977686, /*string->Symbol*/
    FN_StringToRawstring = 7642864381694697020, /*string->rawstring*/
    FN_IsSymbol = 500394459360595447, /*symbol?*/
    FN_SyntaxToAnchor = 15831109139860925413, /*sugar->anchor*/
    FN_SyntaxToDatum = 6995866597099415633, /*sugar->datum*/
    FN_SyntaxCons = 13171793590456702044, /*sugar-cons*/
    FN_SyntaxDo = 6254642707582329165, /*sugar-do*/
    FN_IsSyntaxHead = 6875003728747858504, /*sugar-head?*/
    FN_SyntaxList = 876748717826460774, /*sugar-list*/
    FN_IsSyntaxQuoted = 2489100591013846798, /*sugar-quoted?*/
    FN_SyntaxUnquote = 16141640285832759608, /*sugar-unquote*/
    FN_SymbolToString = 13549505601694953247, /*Symbol->string*/
    FN_StringMatch = 14540312376280599852, /*string-match?*/
    FN_SuperOf = 9665425379163397469, /*superof*/
    FN_SyntaxNew = 8193416659814714084, /*sugar-new*/
    FN_SyntaxWrap = 10245630139626860427, /*sugar-wrap*/
    FN_SyntaxStrip = 3609366682088617090, /*sugar-strip*/
    FN_Translate = 10114311765945214478, /*translate*/
    FN_TupleOf = 13713282200000249508, /*tupleof*/
    FN_TypeNew = 11614088411385566596, /*type-new*/
    FN_TypeName = 15356547873585609923, /*type-name*/
    FN_TypeSizeOf = 2244317750748691605, /*type-sizeof*/
    FN_Typify = 16911065318293036033, /*__typify*/
    FN_TypeEq = 4252330631772254133, /*type==*/
    FN_IsType = 6788264961597143211, /*type?*/
    FN_TypeKind = 8525267428186659104, /*type-kind*/
    FN_TypeDebugABI = 5531674602682195901, /*type-debug-abi*/
    FN_RuntimeTypeAt = 5741048204499520801, /*runtime-type@*/
    FN_VectorOf = 9175027470522679380, /*vectorof*/
    FN_XPCall = 4620798237689849015, /*xpcall*/
    FN_Zip = 5374409871230947730, /*zip*/
    FN_VectorType = 4314115723014650849, /*vector-type*/
    FN_ZipFill = 10943405962239056919, /*zip-fill*/
    SFXFN_CopyMemory = 6123383508381365031, /*copy-memory*/
    SFXFN_Error = 14009282717201935276, /*__error*/
    SFXFN_AnchorError = 10111530143858091593, /*__anchor-error*/
    SFXFN_Abort = 14584786255663284495, /*abort*/
    SFXFN_CompilerError = 5322510740302878417, /*compiler-error*/
    SFXFN_SetAnchor = 12725932041170064112, /*set-anchor*/
    SFXFN_LabelAppendParameter = 7445215136129063900, /*label-append-parameter*/
    SFXFN_RefSet = 17946827824142132549, /*ref-set*/
    SFXFN_SetExceptionHandler = 12886656894248473748, /*set-exception-handler*/
    SFXFN_SetGlobals = 17267486274989061678, /*set-globals*/
    SFXFN_SetTypenameSuper = 17913296430735390683, /*set-typename-super*/
    SFXFN_SetGlobalApplyFallback = 13598598086632150276, /*set-global-apply-fallback*/
    SFXFN_SetScopeSymbol = 4600071934678784730, /*__set-scope-symbol*/
    SFXFN_DelScopeSymbol = 321162543691533479, /*delete-scope-symbol*/
    SFXFN_TranslateLabelBody = 4810009158439512820, /*translate-label-body*/
    OP_NotEq = 3382543495209144820, /*"!= "*/
    OP_Mod = 12140082894871757996, /*%*/
    OP_InMod = 5654219906536963124, /*%=*/
    OP_BitAnd = 6200194738917530187, /*&*/
    OP_InBitAnd = 16592358583153108659, /*&=*/
    OP_IFXMul = 7143879356535221685, /***/
    OP_InMul = 9452546109106220656, /**=*/
    OP_IFXAdd = 12684683064545597449, /*+*/
    OP_Incr = 3527097349658125705, /*++*/
    OP_InAdd = 14257447934227315970, /*+=*/
    OP_Comma = 680152196786002545, /*,*/
    OP_IFXSub = 15041694011100916230, /*-*/
    OP_Decr = 3822003150968230567, /*--*/
    OP_InSub = 7371921654338635201, /*-=*/
    OP_Dot = 9865345230079010229, /*.*/
    OP_Join = 1290004502466654288, /*..*/
    OP_Div = 17985736888374122988, /*/ */
    OP_InDiv = 11465969950632818664, /*/=*/
    OP_Colon = 14527783302615495879, /*:*/
    OP_Let = 12661984279830779943, /*:=*/
    OP_Less = 3641553313283507712, /*<*/
    OP_LeftArrow = 16139754542576582524, /*<-*/
    OP_Subtype = 1505862203851347732, /*<:*/
    OP_ShiftL = 8935135075789616375, /*<<*/
    OP_LessThan = 3759576476138126295, /*<=*/
    OP_Set = 17137173680907217343, /*=*/
    OP_Eq = 2894758336298258323, /*==*/
    OP_Greater = 18073178521569406166, /*>*/
    OP_GreaterThan = 18414829077808003446, /*>=*/
    OP_ShiftR = 7340295454246991699, /*>>*/
    OP_At = 15486603756902508958, /*@*/
    OP_Xor = 5235795292934307304, /*^*/
    OP_InXor = 15846746935447224273, /*^=*/
    OP_And = 7930555440154608777, /*and*/
    OP_Not = 8059530474822141516, /*not*/
    OP_Or = 3273228192728956286, /*or*/
    OP_BitOr = 10192901778115611626, /*|*/
    OP_InBitOr = 3996887634002970672, /*|=*/
    OP_BitNot = 18304183005286158172, /*~*/
    OP_InBitNot = 3502462484992716509, /*~=*/
    SYM_DebugBuild = 8184075142107330479, /*debug-build?*/
    SYM_CompilerDir = 13602281828929908208, /*compiler-dir*/
    SYM_CompilerPath = 13064050785755066828, /*compiler-path*/
    SYM_CompilerTimestamp = 16987829950280636783, /*compiler-timestamp*/
    SYM_WorkingDir = 5447173786803930970, /*working-dir*/
    SYM_CacheDir = 15459783405512119053, /*cache-dir*/
    SYM_Struct = 16789860441568455430, /*struct*/
    SYM_Union = 15260961870135870979, /*union*/
    SYM_TypeDef = 14745078349084699342, /*typedef*/
    SYM_Enum = 12502346185365291857, /*enum*/
    SYM_Array = 15532861776937018509, /*array*/
    SYM_Vector = 15301980077169824507, /*vector*/
    SYM_FNType = 11120704001715159398, /*fntype*/
    SYM_Extern = 6179361444275178947, /*extern*/
    SYM_Const = 6512349124221094905, /*const*/
    Style_None = 8850124634912351594, /*style-none*/
    Style_Symbol = 10901144089013221738, /*style-symbol*/
    Style_String = 14529767959119203830, /*style-string*/
    Style_Number = 2625009603178514204, /*style-number*/
    Style_Keyword = 16897399071257699518, /*style-keyword*/
    Style_Function = 11236791519925792902, /*style-function*/
    Style_SfxFunction = 16716756346950547342, /*style-sfxfunction*/
    Style_Operator = 1919777149988332825, /*style-operator*/
    Style_Instruction = 369412668883625494, /*style-instruction*/
    Style_Type = 18065253812479858713, /*style-type*/
    Style_Comment = 1208308584668339522, /*style-comment*/
    Style_Error = 16775441925030749655, /*style-error*/
    Style_Warning = 9082242703191893382, /*style-warning*/
    Style_Location = 7157884112338744933, /*style-location*/
    SYM_FnCCForm = 3777902970773655997, /*form-fn-body*/
    SYM_DoForm = 5252675358929451147, /*form-do*/
    SYM_SyntaxScope = 18311234245682069372, /*sugar-scope*/
    SYM_CallHandler = 15276107857773597095, /*__call*/
    SYM_ReturnHandler = 13433859184880648379, /*__return*/
    SYM_CopyHandler = 7883394949820752202, /*__copy*/
    SYM_DerefHandler = 11092951210525075234, /*__deref*/
    SYM_QuoteHandler = 13110733606798731532, /*__quote*/
    SYM_UnionFields = 12100360902915947303, /*__fields*/
    SYM_Parenthesis = 4401838540670831418, /*...*/
    SYM_ListWildcard = 6687135905450235841, /*#list*/
    SYM_SymbolWildcard = 7369549451086574715, /*#symbol*/
    SYM_ThisFnCC = 6585467542806116370, /*#this-fn/cc*/
    SYM_HiddenInline = 11618359929726500913, /*#hidden*/
    SYM_Compare = 10304478732946374108, /*compare*/
    SYM_Size = 4336610694859342496, /*size*/
    SYM_Alignment = 5119585074165922935, /*alignment*/
    SYM_Unsigned = 16002710089674172314, /*unsigned*/
    SYM_Bitwidth = 3434266056705820916, /*bitwidth*/
    SYM_Super = 10620761602627926192, /*super*/
    SYM_ApplyType = 17126176567583545790, /*apply-type*/
    SYM_ScopeCall = 3502974719541747140, /*scope-call*/
    SYM_Styler = 10490359231595000575, /*styler*/
    SYM_SquareList = 13757374353737725634, /*square-list*/
    SYM_CurlyList = 4980268125050354152, /*curly-list*/
    SYM_Variadic = 9763876532611965570, /*variadic*/
    SYM_Pure = 9684860523887177582, /*pure*/
    SYM_TargetVertex = 265535381341653816, /*vertex*/
    SYM_TargetFragment = 7717339467431799751, /*fragment*/
    SYM_TargetGeometry = 14861888538789983223, /*geometry*/
    SYM_TargetCompute = 17091827742206395460, /*compute*/
    SYM_Location = 17075619572607297596, /*location*/
    SYM_Binding = 17619849801185222382, /*binding*/
    SYM_Storage = 5407348570825356644, /*storage*/
    SYM_Buffer = 5068647896154219854, /*buffer*/
    SYM_Coherent = 11399022565766744209, /*coherent*/
    SYM_Restrict = 869175600013586879, /*restrict*/
    SYM_ReadOnly = 18105022939393475328, /*readonly*/
    SYM_WriteOnly = 2190819671843390192, /*writeonly*/
    SYM_C = 2297251407937923976, /*c*/
    SYM_Skip = 3974552321822833182, /*skip*/
    SYM_Original = 12586604343039158779, /*original*/
    SYM_Help = 9716048943680351752, /*help*/
    TIMER_Compile = 8730744462762296966, /*compile()*/
    TIMER_CompileSPIRV = 11124148362860993190, /*compile_spirv()*/
    TIMER_Generate = 953501731572172123, /*generate()*/
    TIMER_GenerateSPIRV = 8748589717044592668, /*generate_spirv()*/
    TIMER_Optimize = 9380244355018091705, /*build_and_run_opt_passes()*/
    TIMER_ValidateScope = 3191009994294055459, /*validate_scope()*/
    TIMER_Main = 14377064242579732968, /*main()*/
    TIMER_Specialize = 8785915789363029544, /*specialize()*/
    TIMER_Expand = 4578321004184843145, /*expand()*/
    TIMER_Tracker = 18242090996254185928, /*track()*/
    TIMER_ImportC = 18185955364440927265, /*import_c()*/
    TIMER_Unknown = 10292951226296906324, /*unknown*/
    SYM_ExecuteReturn = 5647993537412616162, /*execute-return*/
    SYM_RCompare = 12328790273400002224, /*rcompare*/
    SYM_CountOfForwarder = 5678758144582795420, /*countof-forwarder*/
    SYM_SliceForwarder = 15326689214775458342, /*slice-forwarder*/
    SYM_JoinForwarder = 14171559422791437536, /*join-forwarder*/
    SYM_RCast = 11546523634840785622, /*rcast*/
    SYM_ROp = 8744847485422937816, /*rop*/
    SYM_CompareListNext = 11722068485195982584, /*compare-list-next*/
    SYM_ReturnSafecall = 12946073503047828647, /*return-safecall*/
    SYM_ReturnError = 7298181064538772091, /*return-error*/
    SYM_XPCallReturn = 10865218228559896416, /*xpcall-return*/
    SYM_SPIRV_StorageClassUniformConstant = 3819476354458542814, /*UniformConstant*/
    SYM_SPIRV_StorageClassInput = 8387700146665956822, /*Input*/
    SYM_SPIRV_StorageClassUniform = 16703291839146902049, /*Uniform*/
    SYM_SPIRV_StorageClassOutput = 15629814936711621602, /*Output*/
    SYM_SPIRV_StorageClassWorkgroup = 14741076744038123277, /*Workgroup*/
    SYM_SPIRV_StorageClassCrossWorkgroup = 11474533878754684672, /*CrossWorkgroup*/
    SYM_SPIRV_StorageClassPrivate = 3518949031431260966, /*Private*/
    SYM_SPIRV_StorageClassFunction = 13837193718927994292, /*Function*/
    SYM_SPIRV_StorageClassGeneric = 11363804437708309977, /*Generic*/
    SYM_SPIRV_StorageClassPushConstant = 14953576086002661032, /*PushConstant*/
    SYM_SPIRV_StorageClassAtomicCounter = 10136563897293808976, /*AtomicCounter*/
    SYM_SPIRV_StorageClassImage = 17666082052013701986, /*Image*/
    SYM_SPIRV_StorageClassStorageBuffer = 10702076198650661246, /*StorageBuffer*/
    SYM_SPIRV_BuiltInPosition = 2382194384671344618, /*spirv.Position*/
    SYM_SPIRV_BuiltInPointSize = 8271161659049525057, /*spirv.PointSize*/
    SYM_SPIRV_BuiltInClipDistance = 17247887399032852954, /*spirv.ClipDistance*/
    SYM_SPIRV_BuiltInCDistance = 16176353722550266875, /*spirv.CDistance*/
    SYM_SPIRV_BuiltInVertexId = 10982206592012632387, /*spirv.VertexId*/
    SYM_SPIRV_BuiltInInstanceId = 10162242499187578702, /*spirv.InstanceId*/
    SYM_SPIRV_BuiltInPrimitiveId = 16772858041657725406, /*spirv.PrimitiveId*/
    SYM_SPIRV_BuiltInInvocationId = 3766231749507252508, /*spirv.InvocationId*/
    SYM_SPIRV_BuiltInLayer = 8097218628678549639, /*spirv.Layer*/
    SYM_SPIRV_BuiltInViewportIndex = 9621766570559635047, /*spirv.ViewportIndex*/
    SYM_SPIRV_BuiltInTessLevelOuter = 4267540544594482620, /*spirv.TessLevelOuter*/
    SYM_SPIRV_BuiltInTessLevelInner = 13018835381245890471, /*spirv.TessLevelInner*/
    SYM_SPIRV_BuiltInTessCoord = 17346565536055819978, /*spirv.TessCoord*/
    SYM_SPIRV_BuiltInPatchVertices = 12551242064588868095, /*spirv.PatchVertices*/
    SYM_SPIRV_BuiltInFragCoord = 12008156278466895563, /*spirv.FragCoord*/
    SYM_SPIRV_BuiltInPointCoord = 6291083464149408266, /*spirv.PointCoord*/
    SYM_SPIRV_BuiltInFrontFacing = 3111370965279528438, /*spirv.FrontFacing*/
    SYM_SPIRV_BuiltInSampleId = 2014163094032111614, /*spirv.SampleId*/
    SYM_SPIRV_BuiltInSamplePosition = 12172088407526591534, /*spirv.SamplePosition*/
    SYM_SPIRV_BuiltInSampleMask = 4132409384659871446, /*spirv.SampleMask*/
    SYM_SPIRV_BuiltInFragDepth = 3878913810878937793, /*spirv.FragDepth*/
    SYM_SPIRV_BuiltInHelperInvocation = 16873999632397810097, /*spirv.HelperInvocation*/
    SYM_SPIRV_BuiltInNumWorkgroups = 16977502600628710055, /*spirv.NumWorkgroups*/
    SYM_SPIRV_BuiltInWorkgroupSize = 11958972563148837083, /*spirv.WorkgroupSize*/
    SYM_SPIRV_BuiltInWorkgroupId = 2799432208558962658, /*spirv.WorkgroupId*/
    SYM_SPIRV_BuiltInLocalInvocationId = 2899496580456011114, /*spirv.LocalInvocationId*/
    SYM_SPIRV_BuiltInGlobalInvocationId = 9456979714347549834, /*spirv.GlobalInvocationId*/
    SYM_SPIRV_BuiltInLocalInvocationIndex = 10248019523169494335, /*spirv.LocalInvocationIndex*/
    SYM_SPIRV_BuiltInWorkDim = 4152321125223184740, /*spirv.WorkDim*/
    SYM_SPIRV_BuiltInGlobalSize = 14534644797400171232, /*spirv.GlobalSize*/
    SYM_SPIRV_BuiltInEnqueuedWorkgroupSize = 1506140060116420169, /*spirv.EnqueuedWorkgroupSize*/
    SYM_SPIRV_BuiltInGlobalOffset = 12830263050193672827, /*spirv.GlobalOffset*/
    SYM_SPIRV_BuiltInGlobalLinearId = 9446286950319146750, /*spirv.GlobalLinearId*/
    SYM_SPIRV_BuiltInSubgroupSize = 2704183859769698851, /*spirv.SubgroupSize*/
    SYM_SPIRV_BuiltInSubgroupMaxSize = 7813807073497713598, /*spirv.SubgroupMaxSize*/
    SYM_SPIRV_BuiltInNumSubgroups = 13826997111228792545, /*spirv.NumSubgroups*/
    SYM_SPIRV_BuiltInNumEnqueuedSubgroups = 3815249322104341692, /*spirv.NumEnqueuedSubgroups*/
    SYM_SPIRV_BuiltInSubgroupId = 9251854604038421458, /*spirv.SubgroupId*/
    SYM_SPIRV_BuiltInSubgroupLocalInvocationId = 4674937656575595786, /*spirv.SubgroupLocalInvocationId*/
    SYM_SPIRV_BuiltInVertexIndex = 14883110553745332205, /*spirv.VertexIndex*/
    SYM_SPIRV_BuiltInInstanceIndex = 13080006323433828190, /*spirv.InstanceIndex*/
    SYM_SPIRV_BuiltInSubgroupEqMaskKHR = 4437109308254469819, /*spirv.SubgroupEqMaskKHR*/
    SYM_SPIRV_BuiltInSubgroupGeMaskKHR = 13628453802632207524, /*spirv.SubgroupGeMaskKHR*/
    SYM_SPIRV_BuiltInSubgroupGtMaskKHR = 16263995048041368826, /*spirv.SubgroupGtMaskKHR*/
    SYM_SPIRV_BuiltInSubgroupLeMaskKHR = 16317328783335416518, /*spirv.SubgroupLeMaskKHR*/
    SYM_SPIRV_BuiltInSubgroupLtMaskKHR = 4864090174588124514, /*spirv.SubgroupLtMaskKHR*/
    SYM_SPIRV_BuiltInBaseVertex = 16808536146664036967, /*spirv.BaseVertex*/
    SYM_SPIRV_BuiltInBaseInstance = 11176430991220786027, /*spirv.BaseInstance*/
    SYM_SPIRV_BuiltInDrawIndex = 6209306614515895236, /*spirv.DrawIndex*/
    SYM_SPIRV_BuiltInDeviceIndex = 907148989857420367, /*spirv.DeviceIndex*/
    SYM_SPIRV_BuiltInViewIndex = 15664288956483665409, /*spirv.ViewIndex*/
    SYM_SPIRV_BuiltInBaryCoordNoPerspAMD = 5877999220324881415, /*spirv.BaryCoordNoPerspAMD*/
    SYM_SPIRV_BuiltInBaryCoordNoPerspCentroidAMD = 9818787022764588388, /*spirv.BaryCoordNoPerspCentroidAMD*/
    SYM_SPIRV_BuiltInBaryCoordNoPerspSampleAMD = 1062506395612045486, /*spirv.BaryCoordNoPerspSampleAMD*/
    SYM_SPIRV_BuiltInBaryCoordSmoothAMD = 5986053031088659732, /*spirv.BaryCoordSmoothAMD*/
    SYM_SPIRV_BuiltInBaryCoordSmoothCentroidAMD = 10818140237770841939, /*spirv.BaryCoordSmoothCentroidAMD*/
    SYM_SPIRV_BuiltInBaryCoordSmoothSampleAMD = 6161870924186642386, /*spirv.BaryCoordSmoothSampleAMD*/
    SYM_SPIRV_BuiltInBaryCoordPModelAMD = 8778250926413290, /*spirv.BaryCoordPModelAMD*/
    SYM_SPIRV_BuiltInViewportMaskNV = 13119906644554086715, /*spirv.ViewportMaskNV*/
    SYM_SPIRV_BuiltInSecondaryPositionNV = 1024936150903929645, /*spirv.SecondaryPositionNV*/
    SYM_SPIRV_BuiltInSecondaryViewportMaskNV = 12443860039389734333, /*spirv.SecondaryViewportMaskNV*/
    SYM_SPIRV_BuiltInPositionPerViewNV = 18188303577941670784, /*spirv.PositionPerViewNV*/
    SYM_SPIRV_BuiltInViewportMaskPerViewNV = 6480247999298724949, /*spirv.ViewportMaskPerViewNV*/
    SYM_SPIRV_ExecutionModeInvocations = 3025613815027394406, /*Invocations*/
    SYM_SPIRV_ExecutionModeSpacingEqual = 5661911267737045454, /*SpacingEqual*/
    SYM_SPIRV_ExecutionModeSpacingFractionalEven = 4604947337916695756, /*SpacingFractionalEven*/
    SYM_SPIRV_ExecutionModeSpacingFractionalOdd = 14144804759298869642, /*SpacingFractionalOdd*/
    SYM_SPIRV_ExecutionModeVertexOrderCw = 5633322680251802084, /*VertexOrderCw*/
    SYM_SPIRV_ExecutionModeVertexOrderCcw = 14709899837782410055, /*VertexOrderCcw*/
    SYM_SPIRV_ExecutionModePixelCenterInteger = 9373731450753457485, /*PixelCenterInteger*/
    SYM_SPIRV_ExecutionModeOriginUpperLeft = 3756882454564435331, /*OriginUpperLeft*/
    SYM_SPIRV_ExecutionModeOriginLowerLeft = 16759260136816157869, /*OriginLowerLeft*/
    SYM_SPIRV_ExecutionModeEarlyFragmentTests = 9809094525453045196, /*EarlyFragmentTests*/
    SYM_SPIRV_ExecutionModePointMode = 6350294295455279573, /*PointMode*/
    SYM_SPIRV_ExecutionModeXfb = 6608919680610637814, /*Xfb*/
    SYM_SPIRV_ExecutionModeDepthReplacing = 17455930285176807235, /*DepthReplacing*/
    SYM_SPIRV_ExecutionModeDepthGreater = 12038224879775260002, /*DepthGreater*/
    SYM_SPIRV_ExecutionModeDepthLess = 6360806020285502421, /*DepthLess*/
    SYM_SPIRV_ExecutionModeDepthUnchanged = 8585474623937453355, /*DepthUnchanged*/
    SYM_SPIRV_ExecutionModeLocalSize = 14901813852583162599, /*LocalSize*/
    SYM_SPIRV_ExecutionModeLocalSizeHint = 8784393887427063421, /*LocalSizeHint*/
    SYM_SPIRV_ExecutionModeInputPoints = 12782706324482141697, /*InputPoints*/
    SYM_SPIRV_ExecutionModeInputLines = 17187068389555880866, /*InputLines*/
    SYM_SPIRV_ExecutionModeInputLinesAdjacency = 1869724779720162391, /*InputLinesAdjacency*/
    SYM_SPIRV_ExecutionModeTriangles = 3012830330780576144, /*Triangles*/
    SYM_SPIRV_ExecutionModeInputTrianglesAdjacency = 6086829274673273494, /*InputTrianglesAdjacency*/
    SYM_SPIRV_ExecutionModeQuads = 7044806828455839943, /*Quads*/
    SYM_SPIRV_ExecutionModeIsolines = 14488265913453970512, /*Isolines*/
    SYM_SPIRV_ExecutionModeOutputVertices = 1370098853646216861, /*OutputVertices*/
    SYM_SPIRV_ExecutionModeOutputPoints = 8319214389319791722, /*OutputPoints*/
    SYM_SPIRV_ExecutionModeOutputLineStrip = 1025481657548809940, /*OutputLineStrip*/
    SYM_SPIRV_ExecutionModeOutputTriangleStrip = 4477483589304494042, /*OutputTriangleStrip*/
    SYM_SPIRV_ExecutionModeVecTypeHint = 6936812832692142581, /*VecTypeHint*/
    SYM_SPIRV_ExecutionModeContractionOff = 9469081271814442351, /*ContractionOff*/
    SYM_SPIRV_ExecutionModePostDepthCoverage = 16039870434385741366, /*PostDepthCoverage*/
    SYM_SPIRV_Dim1D = 8036939238176222723, /*1D*/
    SYM_SPIRV_Dim2D = 17167169805665116695, /*2D*/
    SYM_SPIRV_Dim3D = 1909703899269212721, /*3D*/
    SYM_SPIRV_DimCube = 14603133001790672466, /*Cube*/
    SYM_SPIRV_DimRect = 8588125125277406458, /*Rect*/
    SYM_SPIRV_DimBuffer = 11712582127758873842, /*Buffer*/
    SYM_SPIRV_DimSubpassData = 13905255677898973690, /*SubpassData*/
    SYM_SPIRV_ImageFormatUnknown = 1953818679822775619, /*Unknown*/
    SYM_SPIRV_ImageFormatRgba32f = 12308913927085093447, /*Rgba32f*/
    SYM_SPIRV_ImageFormatRgba16f = 2801664456017198748, /*Rgba16f*/
    SYM_SPIRV_ImageFormatR32f = 8920423497500017802, /*R32f*/
    SYM_SPIRV_ImageFormatRgba8 = 12209698784253337461, /*Rgba8*/
    SYM_SPIRV_ImageFormatRgba8Snorm = 6852814912523374692, /*Rgba8Snorm*/
    SYM_SPIRV_ImageFormatRg32f = 2624754261728856757, /*Rg32f*/
    SYM_SPIRV_ImageFormatRg16f = 13020942806746606891, /*Rg16f*/
    SYM_SPIRV_ImageFormatR11fG11fB10f = 5202052075558122130, /*R11fG11fB10f*/
    SYM_SPIRV_ImageFormatR16f = 17951739255322059553, /*R16f*/
    SYM_SPIRV_ImageFormatRgba16 = 15000310235547380724, /*Rgba16*/
    SYM_SPIRV_ImageFormatRgb10A2 = 5539212566722500476, /*Rgb10A2*/
    SYM_SPIRV_ImageFormatRg16 = 16103040736486123973, /*Rg16*/
    SYM_SPIRV_ImageFormatRg8 = 7623059477226702031, /*Rg8*/
    SYM_SPIRV_ImageFormatR16 = 4024849221544928912, /*R16*/
    SYM_SPIRV_ImageFormatR8 = 4762258727175971445, /*R8*/
    SYM_SPIRV_ImageFormatRgba16Snorm = 10878698787562651648, /*Rgba16Snorm*/
    SYM_SPIRV_ImageFormatRg16Snorm = 9024278493001466244, /*Rg16Snorm*/
    SYM_SPIRV_ImageFormatRg8Snorm = 1707011481821712516, /*Rg8Snorm*/
    SYM_SPIRV_ImageFormatR16Snorm = 8803583628727195378, /*R16Snorm*/
    SYM_SPIRV_ImageFormatR8Snorm = 2299553021005295916, /*R8Snorm*/
    SYM_SPIRV_ImageFormatRgba32i = 10161060370861025599, /*Rgba32i*/
    SYM_SPIRV_ImageFormatRgba16i = 16765054051183802423, /*Rgba16i*/
    SYM_SPIRV_ImageFormatRgba8i = 16030570085357643997, /*Rgba8i*/
    SYM_SPIRV_ImageFormatR32i = 16162241178228541144, /*R32i*/
    SYM_SPIRV_ImageFormatRg32i = 10328943731812016539, /*Rg32i*/
    SYM_SPIRV_ImageFormatRg16i = 6187806417625782191, /*Rg16i*/
    SYM_SPIRV_ImageFormatRg8i = 12947691906691548071, /*Rg8i*/
    SYM_SPIRV_ImageFormatR16i = 420756666554598561, /*R16i*/
    SYM_SPIRV_ImageFormatR8i = 5027105641688406145, /*R8i*/
    SYM_SPIRV_ImageFormatRgba32ui = 14079907389530269104, /*Rgba32ui*/
    SYM_SPIRV_ImageFormatRgba16ui = 7223441526377646866, /*Rgba16ui*/
    SYM_SPIRV_ImageFormatRgba8ui = 4170782091325033023, /*Rgba8ui*/
    SYM_SPIRV_ImageFormatR32ui = 16889413835791334424, /*R32ui*/
    SYM_SPIRV_ImageFormatRgb10a2ui = 5597484258177870543, /*Rgb10a2ui*/
    SYM_SPIRV_ImageFormatRg32ui = 8930432795992272341, /*Rg32ui*/
    SYM_SPIRV_ImageFormatRg16ui = 8363728841036299681, /*Rg16ui*/
    SYM_SPIRV_ImageFormatRg8ui = 16366452795215817663, /*Rg8ui*/
    SYM_SPIRV_ImageFormatR16ui = 10573144600323957652, /*R16ui*/
    SYM_SPIRV_ImageFormatR8ui = 15970624673455669338, /*R8ui*/
    SYM_SPIRV_ImageOperandBias = 4300077647597269850, /*Bias*/
    SYM_SPIRV_ImageOperandLod = 4398168945738258621, /*Lod*/
    SYM_SPIRV_ImageOperandGradX = 500678135338139409, /*GradX*/
    SYM_SPIRV_ImageOperandGradY = 16693532667924534315, /*GradY*/
    SYM_SPIRV_ImageOperandConstOffset = 16637315074023520911, /*ConstOffset*/
    SYM_SPIRV_ImageOperandOffset = 14996469742838170108, /*Offset*/
    SYM_SPIRV_ImageOperandConstOffsets = 9225445972833429128, /*ConstOffsets*/
    SYM_SPIRV_ImageOperandSample = 8688846412954744046, /*Sample*/
    SYM_SPIRV_ImageOperandMinLod = 11327865876307996750, /*MinLod*/
    SYM_SPIRV_ImageOperandDref = 5811046785470675882, /*Dref*/
    SYM_SPIRV_ImageOperandProj = 5466346711829238600, /*Proj*/
    SYM_SPIRV_ImageOperandFetch = 6316873032805425376, /*Fetch*/
    SYM_SPIRV_ImageOperandGather = 8321422982310867990, /*Gather*/
    SYM_SPIRV_ImageOperandSparse = 3976057216718661113, /*Sparse*/
}

impl KnownSymbol {
}