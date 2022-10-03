#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Icon {
    Up = '↑' as u32,
    Save = '💾' as u32,
    Interrobang = '‽' as u32,
    AddBox = '⊞' as u32, // 
    MinusBox = '⊟' as u32, // 
    Undo = '↩' as u32, // 
    Redo = '↪' as u32, // 
    MediaRewind = '⏪' as u32, // 
    MediaPrevious = '⏮' as u32, // 
    MediaNext = '⏭' as u32, // 
    MediaFastforward = '⏩' as u32, //
    MediaStop = '⏹' as u32, // 
    MediaPause = '⏸' as u32, // 
    MediaRecord = '⏺' as u32, // 
    GPSIndicator = '⌖' as u32, // 
    Hourglass = '⌛' as u32, // 
    Square = '■' as u32, // 
    InnerSquare = '▣' as u32, // 
    LeftTriangle = '◀' as u32, // 
    RightTriangle = '▶' as u32, // 
    Diamond = '◊' as u32, // 
    OpenCircle = '○' as u32, // 
    Target = '◎' as u32, // 
    HalfCircle = '◑' as u32, // 
    QuarterCircle = '◔' as u32, // 
    ThreeQuarterCircle = '◕' as u32, // 
    OpenSquare = '◻' as u32, // 
    FilledSquare = '◼' as u32, // 
    Sun = '☀' as u32, // 
    Cloud = '☁' as u32, // 
    Umbrella = '☂' as u32, // 
    UmbrellaRainy = '☔' as u32, // 
    Coffee = '☕' as u32, // 
    Shamrock = '☘' as u32, // 
    Comet = '☄' as u32, // 
    FilledStar = '★' as u32, // 
    OpenStar = '☆' as u32, // 
    CircleDot = '☉' as u32, // 
    Ballot = '☐' as u32, // 
    BallotCheck = '☑' as u32, // 
    LeftPointer = '☜' as u32, // 
    UpPointer = '☝' as u32, // 
    RightPointer = '☞' as u32, // 
    DownPointer = '☟' as u32, // 
    SkullAndBones = '☠' as u32, // 
    RadioActive = '☢' as u32, // 
    Biohazard = '☣' as u32, // 
    Caduceus = '☤' as u32, // 
    Ankh = '☥' as u32, // 
    OrthodoxCross = '☦' as u32, // 
    ChiRho = '☧' as u32, // 
    CrossOfLorraine = '☨' as u32, // 
    CrossOfJerusalem = '☩' as u32, // 
    StarAndCrescent = '☪' as u32, // 
    FarsiSymbol = '☫' as u32, // 
    AdiShakti = '☬' as u32, // 
    HammerAndSickle = '☭' as u32, // 
    Peace = '☮' as u32, // 
    YinYang = '☯' as u32, // 
    HeavenTrigram = '☰' as u32, // 
    WheelOfDharma = '☸' as u32, // 
    SadFace = '☹' as u32, // 
    HappyFace = '☺' as u32, // 
    Female = '♀' as u32, // 
    Male = '♂' as u32, // 
    /// ♈ : 9800
    Aries = '♈' as u32,
    /// ♉ : 9801
    Taurus = '♉' as u32,
    /// ♊ : 9802
    Gemini = '♊' as u32,
    /// ♋ : 9803
    Cancer = '♋' as u32,
    /// ♌ : 9804
    Leo = '♌' as u32,
    /// ♍ : 9805
    Virgo = '♍' as u32,
    /// ♎ : 9806
    Libra = '♎' as u32,
    /// ♏ : 9807
    Scorpius = '♏' as u32,
    /// ♐ : 9808
    Sagitarius = '♐' as u32,
    /// ♑ : 9809
    Capricorn = '♑' as u32,
    /// ♒ : 9810
    Aquarius = '♒' as u32,
    /// ♓ : 9811
    Pisces = '♓' as u32,
    /// ♔ : 9812
    WhiteKing = '♔' as u32,
    /// ♕ : 9813
    WhiteQueen = '♕' as u32,
    /// ♖ : 9814
    WhiteRook = '♖' as u32,
    /// ♗ : 9815
    WhiteBishop = '♗' as u32,
    /// ♘ : 9816
    WhiteKnight = '♘' as u32,
    /// ♙ : 9817
    WhitePawn = '♙' as u32,
    /// ♚ : 9818
    BlackKing = '♚' as u32,
    /// ♛ : 9819
    BlackQueen = '♛' as u32,
    /// ♜ : 9820
    BlackRook = '♜' as u32,
    /// ♝ : 9821
    BlackBishop = '♝' as u32,
    /// ♞ : 9822
    BlackKnight = '♞' as u32,
    /// ♟ : 9823
    BlackPawn = '♟' as u32,
    /// ♠ : 9824
    BlackSpade = '♠' as u32,
    /// ♡ : 9825
    WhiteHeart = '♡' as u32,
    /// ♣ : 9827
    BlackClub = '♣' as u32,
    /// ♥ : 9829
    BlackHeart = '♥' as u32,
    /// ♦ : 9830
    BlackDiamond = '♦' as u32,
    /// ♨ : 9832
    Java = '♨' as u32,
    /// ♩ : 9833
    QuarterNote = '♩' as u32,
    /// ♪ : 9834
    EighthNote = '♪' as u32,
    /// ♫ : 9835
    BeamedEighthNotes = '♫' as u32,
    /// ♬ : 9836
    BeamedSixteenthNotes = '♬' as u32,
    /// ♻ : 9851
    BlackRecyclingLogo = '♻' as u32,
    /// ♾ : 9854
    CircledInfinity = '♾' as u32,
    /// ♿ : 9855
    Wheelchair = '♿' as u32,
    /// ⚐ : 9872
    WhiteFlag = '⚐' as u32,
    /// ⚑ : 9873
    BlackFlag = '⚑' as u32,
    /// ⚒ : 9874
    HammerAndPick = '⚒' as u32,
    /// ⚓ : 9875
    Anchor = '⚓' as u32,
    /// ⚔ : 9876
    CrossedSwords = '⚔' as u32,
    /// ⚕ : 9877
    StaffOfAesculapius = '⚕' as u32,
    /// ⚖ : 9878
    Scales = '⚖' as u32,
    /// ⚘ : 9880
    Flower = '⚘' as u32,
    /// ⚙ : 9881
    Gear = '⚙' as u32,
    /// ⚛ : 9883
    Atom = '⚛' as u32,
    /// ⚜ : 9884
    FleurDeLis = '⚜' as u32,
    /// ⚠ : 9888
    Warning = '⚠' as u32,
    /// ⚡ : 9889
    Voltage = '⚡' as u32,
    /// ⚢ : 9890
    DoubleFemale = '⚢' as u32,
    /// ⚣ : 9891
    DoubleMale = '⚣' as u32,
    /// ⚤ : 9892
    MaleFemale = '⚤' as u32,
    /// ⚦ : 9894
    MaleWithStroke = '⚦' as u32,
    /// ⚧ : 9895
    Transgender = '⚧' as u32,
    /// ⚫ : 9899
    MediumBlackCircle = '⚫' as u32,
    /// ⚪ : 9898
    MediumWhiteCircle = '⚪' as u32,
    /// ⚰ : 9904
    Coffin = '⚰' as u32,
    /// ⚽ : 9917
    SoccerBall = '⚽' as u32,
    /// ⚾ : 9918
    Baseball = '⚾' as u32,
    /// ⛃ : 9923
    Barrel = '⛃' as u32,
    /// ⛓ : 9939
    Chains = '⛓' as u32,
    /// ⛔ : 9940
    NoEntry = '⛔' as u32,
    /// ⛤ : 9956
    Pentagram = '⛤' as u32,
    /// ⛧ : 9959
    InvertedPentagram = '⛧' as u32,
    /// ⛨ : 9960
    BlackCrossOnShield = '⛨' as u32,
    /// ⛩ : 9961
    ShintoShrine = '⛩' as u32,
    /// ⛪ : 9962
    Church = '⛪' as u32,
    /// ⛭ : 9965
    GearWithHub = '⛭' as u32,
    /// ⛵ : 9973
    Sailboat = '⛵' as u32,
    /// ⛶ : 9974
    Selection = '⛶' as u32,
    /// ⛼ : 9980
    Headstone = '⛼' as u32,
    /// ⛽ : 9981
    Gas = '⛽' as u32,
    /// ✂ : 9986
    Scissors = '✂' as u32,
    /// ✅ : 9989
    WhiteHeavyCheckmark = '✅' as u32,
    /// ✆ : 9990
    TelephoneCircle = '✆' as u32,
    /// ✇ : 9991
    FilmRoll = '✇' as u32,
    /// ✈ : 9992
    Airplane = '✈' as u32,
    /// ✉ : 9993
    Envelope = '✉' as u32,
    /// ✊ : 9994
    RaisedFist = '✊' as u32,
    /// ✌ : 9996
    PeaceHand = '✌' as u32,
    /// ✏ : 9999
    Pencil = '✏' as u32,
    /// ✒ : 10002
    Pen = '✒' as u32,
    /// ✔ : 10004
    HeavyCheckmark = '✔' as u32,
    /// ✚ : 10010
    HeavyGreekCross = '✚' as u32,
    /// ✝ : 10013
    LatinCross = '✝' as u32,
    /// ✟ : 10015
    OutlinedLatinCross = '✟' as u32,
    /// ✠ : 10016
    MalteseCross = '✠' as u32,
    /// ✡ : 10017
    StarOfDavid = '✡' as u32,
    /// ✨ : 10024
    Sparkles = '✨' as u32,
    /// ✪ : 10026
    CircledWhiteStar = '✪' as u32,
    /// ✱ : 10033
    HeavyAsterisk = '✱' as u32,
    /// ✳ : 10035
    EightSpokedAsterisk = '✳' as u32,
    /// ✴ : 10036
    EightPointedStar = '✴' as u32,
    /// ✿ : 10047
    BlackFlorette = '✿' as u32,
    /// ❄ : 10052
    Snowflake = '❄' as u32,
    /// ❇ : 10055
    Sparkle = '❇' as u32,
    /// ❌ : 10060
    CrossMark = '❌' as u32,
    /// ❎ : 10062
    XBlock = '❎' as u32,
    /// ❓ : 10067
    BlackQuestionMark = '❓' as u32,
    /// ❔ : 10068
    WhiteQuestionMark = '❔' as u32,
    /// ❕ : 10069
    WhiteExclamationMark = '❕' as u32,
    /// ❗ : 10071
    HeavyExclamationMark = '❗' as u32,
    /// ❤ : 10084
    HeavyBlackHeart = '❤' as u32,
    /// ➕ : 10133
    HeavyPlus = '➕' as u32,
    /// ➖ : 10134
    HeavyMinus = '➖' as u32,
    /// ➗ : 10135
    HeavyDivide = '➗' as u32,
    /// ✖ : 10006
    HeavyMultiply = '✖' as u32,
    /// ➰ : 10160
    CurlyLoop = '➰' as u32,
    /// ⟲ : 10226
    Anticlockwise = '⟲' as u32,
    /// ⟳ : 10227
    Clockwise = '⟳' as u32,
    /// ⬅ : 11013
    LeftArrow = '⬅' as u32,
    /// ➡ : 10145
    RightArrow = '➡' as u32,
    /// ⬆ : 11014
    UpArrow = '⬆' as u32,
    /// ⬇ : 11015
    DownArrow = '⬇' as u32,
    /// ⬈ : 11016
    NEArrow = '⬈' as u32,
    /// ⬉ : 11017
    NWArrow = '⬉' as u32,
    /// ⬊ : 11018
    SEArrow = '⬊' as u32,
    /// ⬋ : 11019
    SWArrow = '⬋' as u32,
    /// ⬌ : 11020
    HorizontalArrows = '⬌' as u32,
    /// ⬍ : 11021
    VerticleArrows = '⬍' as u32,
    /// ⬛ : 11035
    LargeBlackSquare = '⬛' as u32,
    /// ⬜ : 11036
    LargeWhiteSquare = '⬜' as u32,
    /// ⬟ : 11039
    BlackPentagon = '⬟' as u32,
    /// ⬣ : 11043
    BlackHexagon = '⬣' as u32,
    /// ⭐ : 11088
    WhiteStar = '⭐' as u32,
    /// ⭕ : 11093
    Circle = '⭕' as u32,
    /// ⮈ : 11144
    BackCircle = '⮈' as u32,
    /// ⮊ : 11146
    ForwardCircle = '⮊' as u32,
    /// ⮉ : 11145
    UpCircle = '⮉' as u32,
    /// ⮋ : 11147
    DownCircle = '⮋' as u32,
    ///  : 58881
    StackOverflow = '' as u32,
    ///  : 58882
    Vimeo = '' as u32,
    ///  : 58883
    Twitter = '' as u32,
    ///  : 58884
    Facebook = '' as u32,
    ///  : 58885
    GooglePlus = '' as u32,
    ///  : 58886
    Pinterest = '' as u32,
    ///  : 58887
    Tumblr = '' as u32,
    ///  : 58888
    Linkedin = '' as u32,
    ///  : 58890
    StumbleUpon = '' as u32,
    ///  : 58891
    LastFM = '' as u32,
    ///  : 58893
    Spotify = '' as u32,
    ///  : 58895
    Instagram = '' as u32,
    ///  : 58896
    Dropbox = '' as u32,
    ///  : 58897
    Evernote = '' as u32,
    ///  : 58899
    Skype = '' as u32,
    ///  : 58902
    Paypal = '' as u32,
    ///  : 58904
    Android = '' as u32,
    ///  : 58911
    Windows = '' as u32,
    ///  : 58914
    DeviantArt = '' as u32,
    ///  : 58915
    Steam = '' as u32,
    ///  : 58916
    Github = '' as u32,
    ///  : 58917
    Git = '' as u32,
    ///  : 58919
    Soundcloud = '' as u32,
    ///  : 58920
    Reddit = '' as u32,
    ///  : 58921
    Delicious = '' as u32,
    ///  : 58922
    Chrome = '' as u32,
    ///  : 58923
    Firefox = '' as u32,
    ///  : 58924
    InternetExplorer = '' as u32,
    ///  : 58925
    Opera = '' as u32,
    ///  : 58926
    Safari = '' as u32,
    ///  : 58927
    GoogleDrive = '' as u32,
    ///  : 58928
    Wordpress = '' as u32,
    ///  : 58929
    Joomla = '' as u32,
    ///  : 58930
    LastFM2 = '' as u32,
    ///  : 58931
    Foursquare = '' as u32,
    ///  : 58932
    Yelp = '' as u32,
    ///  : 58934
    Youtube = '' as u32,
    ///  : 61898
    Vine = '' as u32,
    ///  : 63743
    Apple = '' as u32,
    /// ！ : 65281
    Exclamation = '！' as u32,
    /// ＃ : 65283
    Octothorpe = '＃' as u32,
    /// ＄ : 65284
    Dollar = '＄' as u32,
    /// ％ : 65285
    Percent = '％' as u32,
    /// ＆ : 65286
    Ampersand = '＆' as u32,
    /// （ : 65288
    LeftParenthesis = '（' as u32,
    /// ） : 65289
    RightParenthesis = '）' as u32,
    /// ＊ : 65290
    Asterisk = '＊' as u32,
    /// ０ : 65296
    Zero = '０' as u32,
    /// １ : 65297
    One = '１' as u32,
    /// ２ : 65298
    Two = '２' as u32,
    /// ３ : 65299
    Three = '３' as u32,
    /// ４ : 65300
    Four = '４' as u32,
    /// ５ : 65301
    Five = '５' as u32,
    /// ６ : 65302
    Six = '６' as u32,
    /// ７ : 65303
    Seven = '７' as u32,
    /// ８ : 65304
    Eight = '８' as u32,
    /// ９ : 65305
    Nine = '９' as u32,
    /// ？ : 65311
    Question = '？' as u32,
    /// ＠ : 65312
    At = '＠' as u32,
    /// 🇦 : 127462
    A = '🇦' as u32,
    /// 🇧 : 127463
    B = '🇧' as u32,
    /// 🇨 : 127464
    C = '🇨' as u32,
    /// 🇩 : 127465
    D = '🇩' as u32,
    /// 🇪 : 127466
    E = '🇪' as u32,
    /// 🇫 : 127467
    F = '🇫' as u32,
    /// 🇬 : 127468
    G = '🇬' as u32,
    /// 🇭 : 127469
    H = '🇭' as u32,
    /// 🇮 : 127470
    I = '🇮' as u32,
    /// 🇯 : 127471
    J = '🇯' as u32,
    /// 🇰 : 127472
    K = '🇰' as u32,
    /// 🇱 : 127473
    L = '🇱' as u32,
    /// 🇲 : 127474
    M = '🇲' as u32,
    /// 🇳 : 127475
    N = '🇳' as u32,
    /// 🇴 : 127476
    O = '🇴' as u32,
    /// 🇵 : 127477
    P = '🇵' as u32,
    /// 🇶 : 127478
    Q = '🇶' as u32,
    /// 🇷 : 127479
    R = '🇷' as u32,
    /// 🇸 : 127480
    S = '🇸' as u32,
    /// 🇹 : 127481
    T = '🇹' as u32,
    /// 🇺 : 127482
    U = '🇺' as u32,
    /// 🇻 : 127483
    V = '🇻' as u32,
    /// 🇼 : 127484
    W = '🇼' as u32,
    /// 🇽 : 127485
    X = '🇽' as u32,
    /// 🇾 : 127486
    Y = '🇾' as u32,
    /// 🇿 : 127487
    Z = '🇿' as u32,
    /// 🌀 : 127744
    Cyclone = '🌀' as u32,
    /// 🌁 : 127745
    Foggy = '🌁' as u32,
    /// 🌂 : 127746
    ClosedUmbrella = '🌂' as u32,
    /// 🌃 : 127747
    StarryNight = '🌃' as u32,
    /// 🌄 : 127748
    MountainSunrise = '🌄' as u32,
    /// 🌅 : 127749
    Sunrise = '🌅' as u32,
    /// 🌆 : 127750
    Dusk = '🌆' as u32,
    /// 🌇 : 127751
    CitySunset = '🌇' as u32,
    /// 🌈 : 127752
    Rainbow = '🌈' as u32,
    /// 🌉 : 127753
    Bridge = '🌉' as u32,
    /// 🌊 : 127754
    Wave = '🌊' as u32,
    /// 🌋 : 127755
    Volcano = '🌋' as u32,
    /// 🌍 : 127757
    GlobeAfrica = '🌍' as u32,
    /// 🌎 : 127758
    GlobeAmericas = '🌎' as u32,
    /// 🌏 : 127759
    GlobeAsia = '🌏' as u32,
    /// 🌐 : 127760
    MeridianGlobe = '🌐' as u32,
    /// 🌑 : 127761
    NewMoon = '🌑' as u32,
    /// 🌒 : 127762
    WaxingCrescentMoon = '🌒' as u32,
    /// 🌓 : 127763
    FirstQuarterMoon = '🌓' as u32,
    /// 🌔 : 127764
    WaxingGibbousMoon = '🌔' as u32,
    /// 🌕 : 127765
    FullMoon = '🌕' as u32,
    /// 🌖 : 127766
    WaningGibbous = '🌖' as u32,
    /// 🌗 : 127767
    LastQuarterMoon = '🌗' as u32,
    /// 🌘 : 127768
    WaningCrescentMoon = '🌘' as u32,
    /// 🌙 : 127769
    CrescentMon = '🌙' as u32,
    /// 🌚 : 127770
    NewMoonFace = '🌚' as u32,
    /// 🌛 : 127771
    FirstQuarterMoonFace = '🌛' as u32,
    /// 🌜 : 127772
    LastQuarterMoonFace = '🌜' as u32,
    /// 🌝 : 127773
    FullMoonFace = '🌝' as u32,
    /// 🌞 : 127774
    SunFace = '🌞' as u32,
    /// 🌠 : 127776
    ShootingStar = '🌠' as u32,
    /// 🌱 : 127793
    Seedling = '🌱' as u32,
    /// 🌲 : 127794
    Evergreen = '🌲' as u32,
    /// 🌳 : 127795
    Deciduous = '🌳' as u32,
    /// 🌴 : 127796
    Palm = '🌴' as u32,
    /// 🌵 : 127797
    Cactus = '🌵' as u32,
    /// 🌷 : 127799
    Tulip = '🌷' as u32,
    /// 🌸 : 127800
    CherryBlossom = '🌸' as u32,
    /// 🌹 : 127801
    Rose = '🌹' as u32,
    /// 🌺 : 127802
    Hibiscus = '🌺' as u32,
    /// 🌻 : 127803
    Sunflower = '🌻' as u32,
    /// 🌼 : 127804
    Blossom = '🌼' as u32,
    /// 🌽 : 127805
    Corn = '🌽' as u32,
    /// 🌾 : 127806
    EarOfRice = '🌾' as u32,
    /// 🌿 : 127807
    Herb = '🌿' as u32,
    /// 🍀 : 127808
    FourLeafClover = '🍀' as u32,
    /// 🍁 : 127809
    MapleLeaf = '🍁' as u32,
    /// 🍂 : 127810
    FallenLeaf = '🍂' as u32,
    /// 🍄 : 127812
    Mushroom = '🍄' as u32,
    /// 🍅 : 127813
    Tomato = '🍅' as u32,
    /// 🍆 : 127814
    Aubergine = '🍆' as u32,
    /// 🍇 : 127815
    Grapes = '🍇' as u32,
    /// 🍉 : 127817
    Watermelon = '🍉' as u32,
    /// 🍋 : 127819
    Lemon = '🍋' as u32,
    /// 🍌 : 127820
    Banana = '🍌' as u32,
    /// 🍍 : 127821
    Pineapple = '🍍' as u32,
    /// 🍐 : 127824
    Pear = '🍐' as u32,
    /// 🍒 : 127826
    Cherries = '🍒' as u32,
    /// 🍓 : 127827
    Strawberry = '🍓' as u32,
    /// 🎃 : 127875
    Halloween = '🎃' as u32,
    /// 🎁 : 127873
    Gift = '🎁' as u32,
    /// 🎂 : 127874
    BirthdayCake = '🎂' as u32,
    /// 🎄 : 127876
    Christmas = '🎄' as u32,
    /// 🎅 : 127877
    Santa = '🎅' as u32,
    /// 🎆 : 127878
    Fireworks = '🎆' as u32,
    /// 🎈 : 127880
    Balloon = '🎈' as u32,
    /// 🎉 : 127881
    PartyPopper = '🎉' as u32,
    /// 🎀 : 127872
    Ribbon = '🎀' as u32,
    /// 🎌 : 127884
    CrossedFlags = '🎌' as u32,
    /// 🎓 : 127891
    GraduationCap = '🎓' as u32,
    /// 🎞 : 127902
    FilmFrames = '🎞' as u32,
    /// 🎟 : 127903
    Ticket = '🎟' as u32,
    /// 💯 : 128175
    Hundred = '💯' as u32,
    /// 💬 : 128172
    Comment = '💬' as u32,
    /// 💭 : 128173
    Thought = '💭' as u32,
    /// 💰 : 128176
    MoneyBag = '💰' as u32,
    /// 💳 : 128179
    CreditCard = '💳' as u32,
    /// 💻 : 128187
    Computer = '💻' as u32,
    /// 📌 : 128204
    Pin = '📌' as u32,
    /// 📍 : 128205
    MapPin = '📍' as u32,
    /// 📎 : 128206
    Paperclip = '📎' as u32,
    /// 📖 : 128214
    OpenBook = '📖' as u32,
    /// 📞 : 128222
    TelephoneReceiver = '📞' as u32,
    /// 📤 : 128228
    OutTray = '📤' as u32,
    /// 📥 : 128229
    InTray = '📥' as u32,
    /// 📦 : 128230
    Package = '📦' as u32,
    /// 📣 : 128227
    CheeringMegaphone = '📣' as u32,
    /// 📢 : 128226
    Intercom = '📢' as u32,
    /// 📡 : 128225
    SateliteAntenna = '📡' as u32,
    /// 📧 : 128231
    Email = '📧' as u32,
    /// 📱 : 128241
    MobilePhone = '📱' as u32,
    /// 📲 : 128242
    ToMobilePhone = '📲' as u32,
    /// 📴 : 128244
    MobilePhoneOff = '📴' as u32,
    /// 📵 : 128245
    NoPhone = '📵' as u32,
    /// 📶 : 128246
    SignalBars = '📶' as u32,
    /// 📷 : 128247
    Camera = '📷' as u32,
    /// 📺 : 128250
    Television = '📺' as u32,
    /// 🔀 : 128256
    Shuffle = '🔀' as u32,
    /// 🔁 : 128257
    Repeat = '🔁' as u32,
    /// 🔂 : 128258
    RepeatOne = '🔂' as u32,
    /// 🔃 : 128259
    RefreshClockwise = '🔃' as u32,
    /// 🔄 : 128260
    RefreshAnticlockwise = '🔄' as u32,
    /// 🔅 : 128261
    LowBrightness = '🔅' as u32,
    /// 🔆 : 128262
    HighBrightness = '🔆' as u32,
    /// 🔇 : 128263
    NoSound = '🔇' as u32,
    /// 🔈 : 128264
    SpeakerLow = '🔈' as u32,
    /// 🔉 : 128265
    SpeakerMedium = '🔉' as u32,
    /// 🔊 : 128266
    SpeakerHigh = '🔊' as u32,
    /// 🔋 : 128267
    Battery = '🔋' as u32,
    /// 🔌 : 128268
    Wire = '🔌' as u32,
    /// 🔍 : 128269
    LeftMagnifier = '🔍' as u32,
    /// 🔎 : 128270
    RightMagnifier = '🔎' as u32,
    /// 🔐 : 128272
    LockAndKey = '🔐' as u32,
    /// 🔑 : 128273
    Key = '🔑' as u32,
    /// 🔔 : 128276
    Bell = '🔔' as u32,
    /// 🔕 : 128277
    NoBell = '🔕' as u32,
    /// 🔙 : 128281
    Back = '🔙' as u32,
    /// 🔚 : 128282
    End = '🔚' as u32,
    /// 🔞 : 128286
    Nsfw = '🔞' as u32,
    /// 🔧 : 128295
    Wrench = '🔧' as u32,
    /// 🔫 : 128299
    Gun = '🔫' as u32,
    /// 🔭 : 128301
    Telescope = '🔭' as u32,
    /// 🔮 : 128302
    CrystalBall = '🔮' as u32,
    /// 🔯 : 128303
    SixPointedStarDot = '🔯' as u32,
    /// 🔱 : 128305
    Trident = '🔱' as u32,
    /// 🕉 : 128329
    Om = '🕉' as u32,
    /// 🕐 : 128336
    OneOClock = '🕐' as u32,
    /// 🕑 : 128337
    TwoOClock = '🕑' as u32,
    /// 🕒 : 128338
    ThreeOClock = '🕒' as u32,
    /// 🕓 : 128339
    FourOClock = '🕓' as u32,
    /// 🕔 : 128340
    FiveOClock = '🕔' as u32,
    /// 🕕 : 128341
    SixOClock = '🕕' as u32,
    /// 🕖 : 128342
    SevenOClock = '🕖' as u32,
    /// 🕗 : 128343
    EightOClock = '🕗' as u32,
    /// 🕘 : 128344
    NineOClock = '🕘' as u32,
    /// 🕙 : 128345
    TenOClock = '🕙' as u32,
    /// 🕚 : 128346
    ElevenOClock = '🕚' as u32,
    /// 🕛 : 128347
    TwelveOClock = '🕛' as u32,
    /// 🕫 : 128363
    Bullhorn = '🕫' as u32,
    /// 🕯 : 128367
    Candle = '🕯' as u32,
    /// 🕱 : 128369
    SkullAndBonesSmall = '🕱' as u32,
    /// 🕷 : 128375
    Spider = '🕷' as u32,
    /// 🖧 : 128423
    Network = '🖧' as u32,
    /// 🖩 : 128425
    Calculator = '🖩' as u32,
    /// 🖮 : 128430
    Keyboard = '🖮' as u32,
    /// 🖱 : 128433
    Mouse = '🖱' as u32,
    /// 🖵 : 128437
    Screen = '🖵' as u32,
    /// 🖶 : 128438
    Printer = '🖶' as u32,
    /// 🖹 : 128441
    TextDocument = '🖹' as u32,
    /// 🗋 : 128459
    EmptyDocument = '🗋' as u32,
    /// 🗀 : 128448
    Folder = '🗀' as u32,
    /// 🗁 : 128449
    OpenFolder = '🗁' as u32,
    /// 🗄 : 128452
    FileCabinet = '🗄' as u32,
    /// 🗊 : 128458
    NotePad = '🗊' as u32,
    /// 🗐 : 128464
    Pages = '🗐' as u32,
    /// 🗑 : 128465
    WasteBasket = '🗑' as u32,
    /// 🗕 : 128469
    Minimize = '🗕' as u32,
    /// 🗖 : 128470
    Maximize = '🗖' as u32,
    /// 🗙 : 128473
    Cancel = '🗙' as u32,
    /// 🗛 : 128475
    Aa = '🗛' as u32,
    /// 🗝 : 128477
    OldKey = '🗝' as u32,
    /// 🗺 : 128506
    WorldMap = '🗺' as u32,
    /// 🚫 : 128683
    NoEntryCircle = '🚫' as u32,
    /// 🚩 : 128681
    Flag = '🚩' as u32,
    /// 🚬 : 128684
    Smoking = '🚬' as u32,
    /// 🚭 : 128685
    NoSmoking = '🚭' as u32,
    /// 󾠬 : 1042476
    OctothorpeButton = '󾠬' as u32,
    /// 󾠮 : 1042478
    Keypad1 = '󾠮' as u32,
    /// 󾠯 : 1042479
    Keypad2 = '󾠯' as u32,
    /// 󾠰 : 1042480
    Keypad3 = '󾠰' as u32,
    /// 󾠱 : 1042481
    Keypad4 = '󾠱' as u32,
    /// 󾠲 : 1042482
    Keypad5 = '󾠲' as u32,
    /// 󾠳 : 1042483
    Keypad6 = '󾠳' as u32,
    /// 󾠴 : 1042484
    Keypad7 = '󾠴' as u32,
    /// 󾠵 : 1042485
    Keypad8 = '󾠵' as u32,
    /// 󾠶 : 1042486
    Keypad9 = '󾠶' as u32,
    /// 󾠷 : 1042487
    Keypad0 = '󾠷' as u32,
    /// ℹ : 8505
    Info = 'ℹ' as u32,
}