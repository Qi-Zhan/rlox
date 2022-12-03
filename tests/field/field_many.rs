use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
class Foo {}

var foo = Foo();
fun setFields() {
  foo.bilberry = "bilberry";
  foo.lime = "lime";
  foo.elderberry = "elderberry";
  foo.raspberry = "raspberry";
  foo.gooseberry = "gooseberry";
  foo.longan = "longan";
  foo.mandarine = "mandarine";
  foo.kiwifruit = "kiwifruit";
  foo.orange = "orange";
  foo.pomegranate = "pomegranate";
  foo.tomato = "tomato";
  foo.banana = "banana";
  foo.juniper = "juniper";
  foo.damson = "damson";
  foo.blackcurrant = "blackcurrant";
  foo.peach = "peach";
  foo.grape = "grape";
  foo.mango = "mango";
  foo.redcurrant = "redcurrant";
  foo.watermelon = "watermelon";
  foo.plumcot = "plumcot";
  foo.papaya = "papaya";
  foo.cloudberry = "cloudberry";
  foo.rambutan = "rambutan";
  foo.salak = "salak";
  foo.physalis = "physalis";
  foo.huckleberry = "huckleberry";
  foo.coconut = "coconut";
  foo.date = "date";
  foo.tamarind = "tamarind";
  foo.lychee = "lychee";
  foo.raisin = "raisin";
  foo.apple = "apple";
  foo.avocado = "avocado";
  foo.nectarine = "nectarine";
  foo.pomelo = "pomelo";
  foo.melon = "melon";
  foo.currant = "currant";
  foo.plum = "plum";
  foo.persimmon = "persimmon";
  foo.olive = "olive";
  foo.cranberry = "cranberry";
  foo.boysenberry = "boysenberry";
  foo.blackberry = "blackberry";
  foo.passionfruit = "passionfruit";
  foo.mulberry = "mulberry";
  foo.marionberry = "marionberry";
  foo.plantain = "plantain";
  foo.lemon = "lemon";
  foo.yuzu = "yuzu";
  foo.loquat = "loquat";
  foo.kumquat = "kumquat";
  foo.salmonberry = "salmonberry";
  foo.tangerine = "tangerine";
  foo.durian = "durian";
  foo.pear = "pear";
  foo.cantaloupe = "cantaloupe";
  foo.quince = "quince";
  foo.guava = "guava";
  foo.strawberry = "strawberry";
  foo.nance = "nance";
  foo.apricot = "apricot";
  foo.jambul = "jambul";
  foo.grapefruit = "grapefruit";
  foo.clementine = "clementine";
  foo.jujube = "jujube";
  foo.cherry = "cherry";
  foo.feijoa = "feijoa";
  foo.jackfruit = "jackfruit";
  foo.fig = "fig";
  foo.cherimoya = "cherimoya";
  foo.pineapple = "pineapple";
  foo.blueberry = "blueberry";
  foo.jabuticaba = "jabuticaba";
  foo.miracle = "miracle";
  foo.dragonfruit = "dragonfruit";
  foo.satsuma = "satsuma";
  foo.tamarillo = "tamarillo";
  foo.honeydew = "honeydew";
}

setFields();

fun printFields() {
  print foo.apple; // expect: apple
  print foo.apricot; // expect: apricot
  print foo.avocado; // expect: avocado
  print foo.banana; // expect: banana
  print foo.bilberry; // expect: bilberry
  print foo.blackberry; // expect: blackberry
  print foo.blackcurrant; // expect: blackcurrant
  print foo.blueberry; // expect: blueberry
  print foo.boysenberry; // expect: boysenberry
  print foo.cantaloupe; // expect: cantaloupe
  print foo.cherimoya; // expect: cherimoya
  print foo.cherry; // expect: cherry
  print foo.clementine; // expect: clementine
  print foo.cloudberry; // expect: cloudberry
  print foo.coconut; // expect: coconut
  print foo.cranberry; // expect: cranberry
  print foo.currant; // expect: currant
  print foo.damson; // expect: damson
  print foo.date; // expect: date
  print foo.dragonfruit; // expect: dragonfruit
  print foo.durian; // expect: durian
  print foo.elderberry; // expect: elderberry
  print foo.feijoa; // expect: feijoa
  print foo.fig; // expect: fig
  print foo.gooseberry; // expect: gooseberry
  print foo.grape; // expect: grape
  print foo.grapefruit; // expect: grapefruit
  print foo.guava; // expect: guava
  print foo.honeydew; // expect: honeydew
  print foo.huckleberry; // expect: huckleberry
  print foo.jabuticaba; // expect: jabuticaba
  print foo.jackfruit; // expect: jackfruit
  print foo.jambul; // expect: jambul
  print foo.jujube; // expect: jujube
  print foo.juniper; // expect: juniper
  print foo.kiwifruit; // expect: kiwifruit
  print foo.kumquat; // expect: kumquat
  print foo.lemon; // expect: lemon
  print foo.lime; // expect: lime
  print foo.longan; // expect: longan
  print foo.loquat; // expect: loquat
  print foo.lychee; // expect: lychee
  print foo.mandarine; // expect: mandarine
  print foo.mango; // expect: mango
  print foo.marionberry; // expect: marionberry
  print foo.melon; // expect: melon
  print foo.miracle; // expect: miracle
  print foo.mulberry; // expect: mulberry
  print foo.nance; // expect: nance
  print foo.nectarine; // expect: nectarine
  print foo.olive; // expect: olive
  print foo.orange; // expect: orange
  print foo.papaya; // expect: papaya
  print foo.passionfruit; // expect: passionfruit
  print foo.peach; // expect: peach
  print foo.pear; // expect: pear
  print foo.persimmon; // expect: persimmon
  print foo.physalis; // expect: physalis
  print foo.pineapple; // expect: pineapple
  print foo.plantain; // expect: plantain
  print foo.plum; // expect: plum
  print foo.plumcot; // expect: plumcot
  print foo.pomegranate; // expect: pomegranate
  print foo.pomelo; // expect: pomelo
  print foo.quince; // expect: quince
  print foo.raisin; // expect: raisin
  print foo.rambutan; // expect: rambutan
  print foo.raspberry; // expect: raspberry
  print foo.redcurrant; // expect: redcurrant
  print foo.salak; // expect: salak
  print foo.salmonberry; // expect: salmonberry
  print foo.satsuma; // expect: satsuma
  print foo.strawberry; // expect: strawberry
  print foo.tamarillo; // expect: tamarillo
  print foo.tamarind; // expect: tamarind
  print foo.tangerine; // expect: tangerine
  print foo.tomato; // expect: tomato
  print foo.watermelon; // expect: watermelon
  print foo.yuzu; // expect: yuzu
}

printFields();

"#;

#[test]
fn test_files_field_many() {
    let expected_output = vec!["apple".to_string(),"apricot".to_string(),"avocado".to_string(),"banana".to_string(),"bilberry".to_string(),"blackberry".to_string(),"blackcurrant".to_string(),"blueberry".to_string(),"boysenberry".to_string(),"cantaloupe".to_string(),"cherimoya".to_string(),"cherry".to_string(),"clementine".to_string(),"cloudberry".to_string(),"coconut".to_string(),"cranberry".to_string(),"currant".to_string(),"damson".to_string(),"date".to_string(),"dragonfruit".to_string(),"durian".to_string(),"elderberry".to_string(),"feijoa".to_string(),"fig".to_string(),"gooseberry".to_string(),"grape".to_string(),"grapefruit".to_string(),"guava".to_string(),"honeydew".to_string(),"huckleberry".to_string(),"jabuticaba".to_string(),"jackfruit".to_string(),"jambul".to_string(),"jujube".to_string(),"juniper".to_string(),"kiwifruit".to_string(),"kumquat".to_string(),"lemon".to_string(),"lime".to_string(),"longan".to_string(),"loquat".to_string(),"lychee".to_string(),"mandarine".to_string(),"mango".to_string(),"marionberry".to_string(),"melon".to_string(),"miracle".to_string(),"mulberry".to_string(),"nance".to_string(),"nectarine".to_string(),"olive".to_string(),"orange".to_string(),"papaya".to_string(),"passionfruit".to_string(),"peach".to_string(),"pear".to_string(),"persimmon".to_string(),"physalis".to_string(),"pineapple".to_string(),"plantain".to_string(),"plum".to_string(),"plumcot".to_string(),"pomegranate".to_string(),"pomelo".to_string(),"quince".to_string(),"raisin".to_string(),"rambutan".to_string(),"raspberry".to_string(),"redcurrant".to_string(),"salak".to_string(),"salmonberry".to_string(),"satsuma".to_string(),"strawberry".to_string(),"tamarillo".to_string(),"tamarind".to_string(),"tangerine".to_string(),"tomato".to_string(),"watermelon".to_string(),"yuzu".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
