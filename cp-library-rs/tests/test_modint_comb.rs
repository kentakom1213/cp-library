use cp_library_rs::modint_comb::*;

const SIZE: usize = 5050505;

#[test]
fn test_permutation() {
    let comb = Comb::<998244353>::new(SIZE);

    assert_eq!(comb.perm(5, 3), 60);
    assert_eq!(comb.perm(349192, 183860), 545014128);
    assert_eq!(comb.perm(408260, 141033), 568084541);
    assert_eq!(comb.perm(302385, 90762), 363332267);
    assert_eq!(comb.perm(422790, 410280), 772920431);
    assert_eq!(comb.perm(472062, 319870), 256733754);
    assert_eq!(comb.perm(248533, 73060), 733999468);
    assert_eq!(comb.perm(252319, 143521), 706025716);
    assert_eq!(comb.perm(191800, 6326), 759308997);
    assert_eq!(comb.perm(414251, 235179), 672952687);
    assert_eq!(comb.perm(31353, 22071), 690599083);
    assert_eq!(comb.perm(95805, 21679), 561322527);
    assert_eq!(comb.perm(408083, 52108), 542443119);
    assert_eq!(comb.perm(469876, 466473), 843578753);
    assert_eq!(comb.perm(107846, 22030), 71461903);
    assert_eq!(comb.perm(328957, 271541), 670944266);
    assert_eq!(comb.perm(360650, 327552), 32074145);
    assert_eq!(comb.perm(473087, 139934), 241607404);
    assert_eq!(comb.perm(364381, 197801), 278234240);
    assert_eq!(comb.perm(437695, 413667), 539537573);
    assert_eq!(comb.perm(189171, 172484), 455977866);
}

#[test]
fn test_combination() {
    let comb = Comb::<998244353>::new(SIZE);

    assert_eq!(comb.comb(259242, 87705), 774328561);
    assert_eq!(comb.comb(71363, 25183), 536450162);
    assert_eq!(comb.comb(475558, 122225), 420488366);
    assert_eq!(comb.comb(368429, 306431), 112920167);
    assert_eq!(comb.comb(395820, 283172), 799630048);
    assert_eq!(comb.comb(274510, 71447), 223324216);
    assert_eq!(comb.comb(429876, 132102), 775071833);
    assert_eq!(comb.comb(407521, 42009), 244493769);
    assert_eq!(comb.comb(438833, 80716), 858062498);
    assert_eq!(comb.comb(470066, 39209), 432758784);
    assert_eq!(comb.comb(415740, 264741), 820821026);
    assert_eq!(comb.comb(434436, 221664), 995889028);
    assert_eq!(comb.comb(130516, 121882), 113770683);
    assert_eq!(comb.comb(486996, 353909), 300270267);
    assert_eq!(comb.comb(478668, 108726), 306915821);
    assert_eq!(comb.comb(414714, 232132), 611611130);
    assert_eq!(comb.comb(393359, 37171), 823768816);
    assert_eq!(comb.comb(416741, 286395), 255449048);
    assert_eq!(comb.comb(296230, 255641), 644480874);
    assert_eq!(comb.comb(384285, 200334), 59276605);
}
