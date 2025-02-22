pub fn zapfdigbats_names_to_unicode(name: &str) -> Option<u16> {
    let names = [
        ("a1", 0x2701),
        ("a10", 0x2721),
        ("a100", 0x275e),
        ("a101", 0x2761),
        ("a102", 0x2762),
        ("a103", 0x2763),
        ("a104", 0x2764),
        ("a105", 0x2710),
        ("a106", 0x2765),
        ("a107", 0x2766),
        ("a108", 0x2767),
        ("a109", 0x2660),
        ("a11", 0x261b),
        ("a110", 0x2665),
        ("a111", 0x2666),
        ("a112", 0x2663),
        ("a117", 0x2709),
        ("a118", 0x2708),
        ("a119", 0x2707),
        ("a12", 0x261e),
        ("a120", 0x2460),
        ("a121", 0x2461),
        ("a122", 0x2462),
        ("a123", 0x2463),
        ("a124", 0x2464),
        ("a125", 0x2465),
        ("a126", 0x2466),
        ("a127", 0x2467),
        ("a128", 0x2468),
        ("a129", 0x2469),
        ("a13", 0x270c),
        ("a130", 0x2776),
        ("a131", 0x2777),
        ("a132", 0x2778),
        ("a133", 0x2779),
        ("a134", 0x277a),
        ("a135", 0x277b),
        ("a136", 0x277c),
        ("a137", 0x277d),
        ("a138", 0x277e),
        ("a139", 0x277f),
        ("a14", 0x270d),
        ("a140", 0x2780),
        ("a141", 0x2781),
        ("a142", 0x2782),
        ("a143", 0x2783),
        ("a144", 0x2784),
        ("a145", 0x2785),
        ("a146", 0x2786),
        ("a147", 0x2787),
        ("a148", 0x2788),
        ("a149", 0x2789),
        ("a15", 0x270e),
        ("a150", 0x278a),
        ("a151", 0x278b),
        ("a152", 0x278c),
        ("a153", 0x278d),
        ("a154", 0x278e),
        ("a155", 0x278f),
        ("a156", 0x2790),
        ("a157", 0x2791),
        ("a158", 0x2792),
        ("a159", 0x2793),
        ("a16", 0x270f),
        ("a160", 0x2794),
        ("a161", 0x2192),
        ("a162", 0x27a3),
        ("a163", 0x2194),
        ("a164", 0x2195),
        ("a165", 0x2799),
        ("a166", 0x279b),
        ("a167", 0x279c),
        ("a168", 0x279d),
        ("a169", 0x279e),
        ("a17", 0x2711),
        ("a170", 0x279f),
        ("a171", 0x27a0),
        ("a172", 0x27a1),
        ("a173", 0x27a2),
        ("a174", 0x27a4),
        ("a175", 0x27a5),
        ("a176", 0x27a6),
        ("a177", 0x27a7),
        ("a178", 0x27a8),
        ("a179", 0x27a9),
        ("a18", 0x2712),
        ("a180", 0x27ab),
        ("a181", 0x27ad),
        ("a182", 0x27af),
        ("a183", 0x27b2),
        ("a184", 0x27b3),
        ("a185", 0x27b5),
        ("a186", 0x27b8),
        ("a187", 0x27ba),
        ("a188", 0x27bb),
        ("a189", 0x27bc),
        ("a19", 0x2713),
        ("a190", 0x27bd),
        ("a191", 0x27be),
        ("a192", 0x279a),
        ("a193", 0x27aa),
        ("a194", 0x27b6),
        ("a195", 0x27b9),
        ("a196", 0x2798),
        ("a197", 0x27b4),
        ("a198", 0x27b7),
        ("a199", 0x27ac),
        ("a2", 0x2702),
        ("a20", 0x2714),
        ("a200", 0x27ae),
        ("a201", 0x27b1),
        ("a202", 0x2703),
        ("a203", 0x2750),
        ("a204", 0x2752),
        ("a205", 0x276e),
        ("a206", 0x2770),
        ("a21", 0x2715),
        ("a22", 0x2716),
        ("a23", 0x2717),
        ("a24", 0x2718),
        ("a25", 0x2719),
        ("a26", 0x271a),
        ("a27", 0x271b),
        ("a28", 0x271c),
        ("a29", 0x2722),
        ("a3", 0x2704),
        ("a30", 0x2723),
        ("a31", 0x2724),
        ("a32", 0x2725),
        ("a33", 0x2726),
        ("a34", 0x2727),
        ("a35", 0x2605),
        ("a36", 0x2729),
        ("a37", 0x272a),
        ("a38", 0x272b),
        ("a39", 0x272c),
        ("a4", 0x260e),
        ("a40", 0x272d),
        ("a41", 0x272e),
        ("a42", 0x272f),
        ("a43", 0x2730),
        ("a44", 0x2731),
        ("a45", 0x2732),
        ("a46", 0x2733),
        ("a47", 0x2734),
        ("a48", 0x2735),
        ("a49", 0x2736),
        ("a5", 0x2706),
        ("a50", 0x2737),
        ("a51", 0x2738),
        ("a52", 0x2739),
        ("a53", 0x273a),
        ("a54", 0x273b),
        ("a55", 0x273c),
        ("a56", 0x273d),
        ("a57", 0x273e),
        ("a58", 0x273f),
        ("a59", 0x2740),
        ("a6", 0x271d),
        ("a60", 0x2741),
        ("a61", 0x2742),
        ("a62", 0x2743),
        ("a63", 0x2744),
        ("a64", 0x2745),
        ("a65", 0x2746),
        ("a66", 0x2747),
        ("a67", 0x2748),
        ("a68", 0x2749),
        ("a69", 0x274a),
        ("a7", 0x271e),
        ("a70", 0x274b),
        ("a71", 0x25cf),
        ("a72", 0x274d),
        ("a73", 0x25a0),
        ("a74", 0x274f),
        ("a75", 0x2751),
        ("a76", 0x25b2),
        ("a77", 0x25bc),
        ("a78", 0x25c6),
        ("a79", 0x2756),
        ("a8", 0x271f),
        ("a81", 0x25d7),
        ("a82", 0x2758),
        ("a83", 0x2759),
        ("a84", 0x275a),
        ("a85", 0x276f),
        ("a86", 0x2771),
        ("a87", 0x2772),
        ("a88", 0x2773),
        ("a89", 0x2768),
        ("a9", 0x2720),
        ("a90", 0x2769),
        ("a91", 0x276c),
        ("a92", 0x276d),
        ("a93", 0x276a),
        ("a94", 0x276b),
        ("a95", 0x2774),
        ("a96", 0x2775),
        ("a97", 0x275b),
        ("a98", 0x275c),
        ("a99", 0x275d),
        ("space", 0x0020),
    ];

    let result = names.binary_search_by_key(&name, |&(name, _code)| &name);
    result.ok().map(|indx| names[indx].1)
}
