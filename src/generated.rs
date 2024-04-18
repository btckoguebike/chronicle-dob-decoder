#![allow(dead_code)]
pub mod character {
    pub const ADJECTIVE: &str = "{\"bytes\":1,\"schema\":{\"simple\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"trait_pool\":[\"TRC_010\",\"TRC_011\",\"TRC_012\",\"TRC_013\",\"TRC_014\",\"TRC_015\",\"TRC_016\"]}}}}";
    pub const NAME: &str = "{\"bytes\":1,\"schema\":{\"simple\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"trait_pool\":[\"TRC_001\",\"TRC_002\",\"TRC_003\",\"TRC_004\",\"TRC_005\",\"TRC_006\"]}}}}";
    pub const PROFESSION: &str = "{\"bytes\":1,\"schema\":{\"simple\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"trait_pool\":[\"TRC_020\",\"TRC_021\",\"TRC_022\",\"TRC_023\"]}}}}";
    pub const HP: &str = "{\"bytes\":1,\"schema\":{\"simple\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_range\":[50,100]}}}}";
    pub const POWER: &str = "{\"bytes\":2,\"schema\":{\"simple\":{\"occupied\":2,\"selector\":\"double\",\"pool\":{\"number_pool\":[3,3,3,3,3,3,3,3,3,4,4,4,4,4,5,5,5]}}}}";
    pub const ATTACK: &str = "{\"bytes\":1,\"schema\":{\"simple\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_range\":[0,30]}}}}";
    pub const DEFENSE: &str = "{\"bytes\":1,\"schema\":{\"simple\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_range\":[0,30]}}}}";
    pub const GOLD: &str = "{\"bytes\":1,\"schema\":{\"simple\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_range\":[0,200]}}}}";
    pub const CARD: &str = "{\"bytes\":14,\"schema\":{\"variable\":{\"number\":{\"occupied\":2,\"selector\":\"double\",\"pool\":{\"number_pool\":[2,2,2,2,3,3]}},\"patterns\":[{\"occupied\":2,\"selector\":\"double\",\"pool\":{\"template_pool\":[\"TEC_001\",\"TEC_002\"]}},{\"occupied\":2,\"selector\":\"double\",\"pool\":{\"template_pool\":[\"TEC_001\",\"TEC_002\"]}},{\"occupied\":2,\"selector\":\"double\",\"pool\":{\"template_pool\":[\"TEC_001\",\"TEC_002\"]}}]}}}";
}
pub mod story {
    pub const CHARACTER: &str = "{\"bytes\":6,\"schema\":{\"multiple_variables\":[{\"number\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_pool\":[0,0,1,1,1,1]}},\"patterns\":[{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"trait_pool\":[\"TRC_010\",\"TRC_011\",\"TRC_012\",\"TRC_013\",\"TRC_014\",\"TRC_015\",\"TRC_016\"]}}]},{\"number\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_pool\":[0,0,1,1,1,1]}},\"patterns\":[{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"trait_pool\":[\"TRC_001\",\"TRC_002\",\"TRC_003\",\"TRC_004\",\"TRC_005\",\"TRC_006\"]}}]},{\"number\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_pool\":[0,0,1,1,1,1]}},\"patterns\":[{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"trait_pool\":[\"TRC_020\",\"TRC_021\",\"TRC_022\",\"TRC_023\"]}}]}]}}";
    pub const LOCATION: &str = "{\"bytes\":6,\"schema\":{\"multiple_variables\":[{\"number\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_pool\":[0]}},\"patterns\":[]},{\"number\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_pool\":[0]}},\"patterns\":[]},{\"number\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_pool\":[0]}},\"patterns\":[]}]}}";
    pub const DATE: &str = "{\"bytes\":6,\"schema\":{\"multiple_variables\":[{\"number\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_pool\":[0]}},\"patterns\":[]},{\"number\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_pool\":[0]}},\"patterns\":[]},{\"number\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_pool\":[0]}},\"patterns\":[]}]}}";
    pub const STORY: &str = "{\"bytes\":4,\"schema\":{\"fixed\":[{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"trait_pool\":[\"PAB01\"]}},{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"trait_pool\":[\"PAT01\",\"PAT02\",\"PAT03\",\"PAT04\",\"PAT05\",\"PAT06\",\"PAT07\",\"PAT08\",\"PAT09\",\"PAT10\",\"PAT11\",\"PAT12\",\"PAT13\",\"PAT14\",\"PAT15\",\"PAT16\",\"PAT17\"]}},{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"trait_pool\":[\"PAC01\",\"PAC02\",\"PAC03\",\"PAC04\",\"PAC05\",\"PAC06\",\"PAC07\",\"PAC08\",\"PAC09\",\"PAC10\",\"PAC11\",\"PAC12\",\"PAC13\",\"PAC14\",\"PAC15\",\"PAC16\"]}},{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"trait_pool\":[\"PAE01\",\"PAE02\",\"PAE03\",\"PAE04\",\"PAE05\",\"PAE06\",\"PAE07\",\"PAE08\",\"PAE09\",\"PAE10\",\"PAE11\",\"PAE12\"]}}]}}";
}
pub mod language {
    pub mod cn {
        pub const TRAIT_POOL: &str = "{\"__CHARACTER_SEGMENTATION_NAME__\":\"__角色名字__\",\"TRC_001\":\"赛尔文\",\"TRC_002\":\"埃尔德林\",\"TRC_003\":\"莫尔丁\",\"TRC_004\":\"泰勒斯\",\"TRC_005\":\"达尼洛斯\",\"TRC_006\":\"卡尔文\",\"__CHARACTER_SEGMENTATION_ADJECTIVE__\":\"__角色描述__\",\"TRC_010\":\"烈焰的\",\"TRC_011\":\"辉夜的\",\"TRC_012\":\"不朽的\",\"TRC_013\":\"传奇的\",\"TRC_014\":\"仙境的\",\"TRC_015\":\"神圣的\",\"TRC_016\":\"无尽的\",\"__CHARACTER_SEGMENTATION_PROFESSION__\":\"__角色职业__\",\"TRC_020\":\"战士\",\"TRC_021\":\"刺客\",\"TRC_022\":\"法师\",\"TRC_023\":\"牧师\"}";
        pub const TEMPLATE_POOL: &str = "{\"TEC_001\":[{\"range\":[0,3]},{\"template\":\"攻击[x]\"},{\"range\":[6,10]},{\"template\":\"造成x点伤害\"}],\"TEC_002\":[{\"range\":[0,3]},{\"template\":\"闪电击[x]\"},{\"range\":[6,12]},{\"template\":\"对所有敌人造成x点伤害\"}]}";
        pub const PARAGRAPH_POOL: &str = "{\"__PARAGRAPH_BEGINING__\":\"__故事开头__\",\"PAB01\":\"{7}{8}{9}，{1}{2}{3}在{4}{5}{6}\",\"__PARAGRAPH_TRANSITION\":\"__故事过渡__\",\"PAT01\":\"之中独自漫步并寻找目标，手中的水晶球闪烁着幽蓝的光芒，预示着即将到来的命运转折，怪物咆哮声由远而近，战斗一触即发。\",\"PAT02\":\"与精灵交换知识,眼睛闪烁着好奇的光芒，渴望学习更多的战斗技巧，没有比怪物更好练手，飞奔向怪兽战斗顷刻展开。\",\"PAT03\":\"与魔法祖先沟通,通过古老的仪式，试图寻找自己的传承和使命，而后竟发现怪物潜伏在面前不远处，幽灵般的身影引入黑暗，恐惧随之蔓延。\",\"PAT04\":\"在探索中发现了一座古老的遗迹。这座遗迹曾经是繁华的象征，但现在却已经被辐射和风沙所掩埋，怪兽腾然出现，舞动着愤怒的双翼。\",\"PAT05\":\"观察着周围的环境，搜索着可能隐藏着危险的角落。收集着各种物品，准备迎接即将到来的挑战，怪兽们靠近了。\",\"PAT06\":\"审视着废墟中的静谧和怪物带来了一股压抑的气息，心中也连带涌动着无尽的勇气,探索和猎杀仍将继续。\",\"PAT07\":\"准备着即将到来的挑战。每次挥舞武器每次踏步前行，都是对自身的考验，也是对战斗的预演，不料成群的怪物已飞奔到面前。\",\"PAT08\":\"屏气秉息面对着如潮水般涌来的敌人，战斗已经就绪，目光同样锐利，非弑尽不还家。\",\"PAT09\":\"闭目凝神，身体逐渐进入一种战斗的状态。装备和武器已准备就绪，只待挑战的到来。\",\"PAT10\":\"深吸一口气,当敌人的影子出现在视野中时攻击也倾泻而出，战斗蔓延狂风乱舞。\",\"PAT11\":\"开启了战斗，到处是金属碰撞的声音和狂暴怪物的咆哮声。怪物展露庞大的身躯和恐怖的力量，试图碾碎一切。\",\"PAT12\":\"踏入古老的殿堂铺面而来一股强大的能量。环顾四周，发现布满了古老的符文和图腾，这预示着接近一个重要的发现。\",\"PAT13\":\"跨过石块和残垣出现充满奥古气息的神秘塔，塔的顶部笼罩着神秘的气息仿佛在发出邀请，似乎踏上阶梯，就能揭开深处的秘密。\",\"PAT14\":\"尝试在战斗中磨砺技巧，带着训练精巧和秘术的每一击都能够在战斗中造成毁灭性的伤害，面对再多敌人也能游刃有余。\",\"PAT15\":\"穿过一片废墟，古老的祭坛出现在了视野中，其上神秘的符文闪着微光，只有脚步回荡的静谧中似乎有着些许躁动。\",\"PAT16\":\"一群凶恶的怪物阻挡了前进的道路,是时候做出决定了：是选择战斗一举突破，还是绕过寻找其他的道路前进。\",\"PAT17\":\"接到了一位神秘人的消息，称有一个重要的任务需要他的帮助。任务是前往废墟深处的迷宫，寻找一件失落的宝物，据说能够拯救这个被废墟笼罩的世界。\",\"__PARAGRAPH_CLIMAX\":\"__故事高潮__\",\"PAC01\":\"战斗的火光在燃烧，突然光芒大盛，一道神秘的遗物出现两者产生了共鸣，还不等反应冲出一只怪物试图阻止靠近遗物，与外来者的大战一触即发。怪物的攻击令人窒息，技巧在苦战中攀升，每一次的无情攻击都在挑战着命运的极限。此刻感受着无穷力量的涌动，随后一道光柱贯穿天地！\",\"PAC02\":\"战斗的火光在燃烧，突然一道天幕冲天而降指引在废墟深处的迷宫，符文能量诉说着失落的宝物痕迹，一道锐鸣破空而来，没有丝毫犹豫火花炸裂随机与怪物展开激烈的战斗。尽管面临着巨大的挑战和风险但信念将能够战胜一切困难，此刻将全部力量灌注，放弃防御释放最强一击！\",\"PAC03\":\"战斗的火光在燃烧，突然传送门亮起，没有过多的考虑旅者将继续游荡。在离开的路上经历反反复复涌上心头，各种奥妙的符文、考验与怪物的力量反复杂糅在一起，真正的挑战从未到来但此刻似乎已经有所突破，盘坐在树下这一切都开始由慢而快的在血脉中流淌，直至隐有风雷之声，顷刻间远处的巨石化为湮粉。\",\"PAC04\":\"战斗的火光在燃烧，突然间符文能量暴动，一道道炫目凛冽的光芒和波动交织，似雷霆地震般轰鸣，将整个天空映照得如同白昼。舞动出的龙卷风袭来，全部生灵顷刻间化为虚无，伫立在暴风眼中，暗影下的身形此刻无比耀眼，这一击终结未来与黑暗！\",\"PAC05\":\"战斗的火光在燃烧，突然狂风骤起，阴冷毁灭的波动迅速袭来，能量恢弘如炬，燃烧着不灭的意志驱散着周围的黑暗，血肉横飞间，偷袭的怪兽已被重创，塌陷的身体几乎被暴走的能量焚毁，心跳如鼓，奥义在此刻凝结！\",\"PAC06\":\"战斗的火光在燃烧，突然耀光划破混沌，孤独的身影在攻击的乱流中穿梭，独自面对着无尽的黑暗，但内心却比黑暗更加深沉，因为知道背后有无数队友期待着久违的胜利，蓄势完毕，挥出这只有胜利的一击！\",\"PAC07\":\"战斗的火光在燃烧，突然黑暗的能量如同潮水般涌来，将一切都笼罩在黑暗之中，使得树木在恐惧，花草在凋零。一丝坚定与决绝从他的身上散发出来,他的身体开始发出微弱的光芒，逐渐变得明亮起来。一股神圣的力量在他体内涌动。\",\"PAC08\":\"战斗的火光在燃烧，突然一道黑色闪电划过天际瞬间袭来。黑袍挡下了黑暗闪电，但其恐怖力量噼啪炸响。攻势越来越猛烈，仿佛要将一切彻底吞噬在黑暗之中，战斗愈发激烈，能量的碰撞在不停地回荡闪烁着正义的光芒。\",\"PAC09\":\"战斗的火光在燃烧，突然怪物的身躯被刺穿，最后一只怪物的身躯倒下时天地寂静。不久又继续震动起来，怪物尚未被打倒狂乱的攻击仍然舞动如狂风，在已无余力发功攻击的时刻，突然意识到获得超越自己的力量需要押上一切的信念与生命，这一击消耗无量的生命，也带来无限的震撼。\",\"PAC10\":\"战斗的火光在燃烧，突然无尽的烈焰在夜空中舞动，攻击的波动如寒光一般激荡，身影如幽灵般穿梭于废墟之间，血雨飘洒，但守护的意志坚不可摧，每一次挥攻击都是一次对邪恶的无情抗击，这愤怒终将吞没所有敌人。\",\"PAC11\":\"战斗的火光在燃烧，突然对抗怪兽身影身上的符文之花闪烁着奇异的光芒，每一朵都散发出治愈的力量。那身躯仿佛是被一层神秘的护盾所保护，在战斗中始终保持着最强大的状态，黑暗中闪烁着剑光与法术的交织，消灭一切来犯之敌。\",\"PAC12\":\"战斗的火光在燃烧，突然绚烂的魔法能量绽放出来，每一次法术都是一次对黑暗力量的粉碎，每一道光芒都是一次对希望的召唤，秘药的力量不断使伤口愈合，对生命重塑，不断地使得可以战斗中保持最强大的战斗状态，直至毁灭性的能量将黑暗化为灰烬。\",\"PAC13\":\"战斗的火光在燃烧，突然地面突然爆发出一道华丽的光幕，飘散着幻化的符文之花，每一朵都散发着神秘的气息。怪物看到面前的人地身体如同被某种神秘力量注入，散发出一种超凡脱俗的气息，仿佛是传说中的神灵降临人间，抬手间怪兽成片的倒下。\",\"PAC14\":\"战斗的火光在燃烧，突然怪物的出现让其陷入了前所未有的绝境，但他并未退缩，反而更加坚定地与怪物展开了一场生死搏斗。能量闪烁，火花四溅，他的身影在废墟中舞动，如同一道不可逾越的光芒，将黑暗一一击溃。\",\"PAC15\":\"战斗的火光在燃烧，突然面对怪兽展现出了惊人的力量和勇气，化身闪电一般在废墟中穿梭，将黑暗一一击溃。无论是怪物的狂暴还是黑暗的侵袭，都无法动摇坚定的信念和不屈的意志。最终以胜利者的姿态站立在废墟之巅，宣告着光明的胜利。\",\"PAC16\":\"战斗的火光在燃烧，突然从废墟的深处传来一阵巨大的轰鸣声，一只庞大而恐怖的怪物出现。它的身躯高耸入云狰狞的面孔充满着暴虐，最大挑战降临。没有时间退缩了，凝聚起内心最深处的力量，划破长空斩向怪兽，巨大的爆炸席卷了一切。\",\"__PARAGRAPH_ENDING__\":\"__故事结尾__\",\"PAE01\":\"尘埃落定之时英勇的身影已消失在远方的天际线上，留下的是一片苍凉的战场，但在背影中人们看到了希望的种子，这是一个新的开始，一起变强吧。\",\"PAE02\":\"尘埃落定之时下起了细雨，世界仿佛俯视着自己的保卫者，尽管战斗已经结束，但守护家园的使命将永不止息，因为正义的火焰将永远燃烧下去。\",\"PAE03\":\"尘埃落定之时未知敌人的咆哮声仍回荡在废墟间，步伐不停像这样的勇士们已是最后一道守护家园的防线，他们的锋芒已经为了家园而磨砺得锋利无比。\",\"PAE04\":\"尘埃落定之时最后一名怪物倒下，英勇的身躯跪在废墟中，周围的黑暗被驱散，一缕阳光透过云层洒在他身上，身躯眼中闪烁着星辰般的光芒，那是胜利的火焰，也是希望的源泉。\",\"PAE05\":\"尘埃落定之时敌人的低吼回响在荒野上，这最后的守护者脚步坚定，战斗或许结束，但保家卫国的使命永远不会停止，孢子战士们将永远坚守在这片土地上。\",\"PAE06\":\"尘埃落定之时身躯缓缓站起，人们清楚虽然战斗结束，但黑暗永远不会消失，孢子战士们将继续使用秘技、秘术与秘药，保卫这片土地，直到最后一滴血液，直到最后一息。\",\"PAE07\":\"尘埃落定之时身躯仍站在废墟之巅，远眺着天际。这一刻心中，既有对过去的回忆也有对未来的期许。冒险与挑战永远不会停止，而作为守护者的使命也将永远延续下去。在黑暗与光明的边缘继续守护着世界的和平与安宁。\",\"PAE08\":\"尘埃落定之时已经恢复了平静，废墟似乎在述说着一段传奇的故事，而这个故事的主角在光明的照耀下踏上了新的征程，继续向着未知的远方前行，直至最后的胜利，直至永恒。他的胜利不仅是对自己勇气和力量的肯定，更是对信念和希望的礼赞。\",\"PAE09\":\"尘埃落定之时怪物最终被击败，废墟中的宁静再度降临，与之一起降临的还有胜利的喜悦和孤独的沉思。在废墟的深处的此刻，孢子战士找到了一枚闪烁着神秘光芒的宝石，它将成为未来冒险的利刃，也是对抗黑暗的信仰。\",\"PAE10\":\"尘埃落定之时孢子战士缓缓站起，虽然战斗结束但黑暗永远不会消失。孢子战士将继续使用秘术与秘药保卫这片土地直到最后一刻。镜头最后定格在背影上，夕阳西下如同一幅永恒的画卷。\",\"PAE11\":\"尘埃落定之时孢子战士的身影已经消失在硝烟弥漫的战场上，他留下的是一片重生的希望，宝物此刻似乎都已经不重要了，因为更具威力的东西已经被掌握了。\",\"PAE12\":\"尘埃落定之时当最后一名怪物倒下，孢子战士陷入了巨大的欣喜中，虽然呼吸急促身上的伤痕无数，虽然探索还有很长的路要走，但很清楚的感知是——确实变强了。\"}";
    }
}
