mod blockchain;
mod config;
mod network;

use colored::Colorize;
use config::Config;
use network::server::Server;
use nexium::{
    blockchain::{data_type::DataType, transaction::Transaction},
    defaults::*,
    gitlab::{GitlabClient, TokenType},
    rsa::KeyPair,
    utils::rand::create_noise,
};
use num_bigint::BigUint;
use std::{env, path::Path, str::FromStr};

const GEN_CONFIG_ARG: &str = "--generate-config";
const DEFAULT_CONFIG_NAME: &str = "config.json";

fn main() {
    // Getting the arguments
    let args = env::args().collect::<Vec<String>>();

    let mut config_path = Path::new(&NEXIUM_HOME).to_path_buf();
    config_path.push(DEFAULT_CONFIG_NAME);

    if !config_path.exists() {
        println!(
            "Config file {} does not exist. Creating the directory... ",
            config_path.to_str().unwrap().red().bold()
        );
        std::fs::create_dir_all(config_path.parent().unwrap())
            .expect("Failed to create config directory");
    } else {
        println!(
            "Config file {} already exists.",
            config_path.to_str().unwrap().red().bold()
        );
    }

    let config = if args.len() > 1 && args[1] == GEN_CONFIG_ARG {
        // Generate the config file
        println!(
            "Argument {} passed.\nGenerating the config in {}... ",
            args[1].bold(),
            config_path.to_str().unwrap().red().bold()
        );

        Config::generate(&config_path)
    } else {
        println!(
            "Looking for the config in {}... ",
            config_path.to_str().unwrap().red().bold()
        );
        Config::from_file(&config_path.clone())
    };

    let gitlab =
        GitlabClient::new(config.gitlab_token.clone(), TokenType::Classic);

    // let mut server = Server::new(&config, &gitlab);
    // server.listen();

    ////////////////////////////////

    // let key = KeyPair::generate(2048, &config.user_login);
    // gitlab.add_gpg_key(key.pub_to_pem().as_str()).unwrap();
    // println!(
    //     "GPG key added to Gitlab for user {}",
    //     config.user_login.green().bold()
    // );

    // println!("Public key: {}", key.pub_to_pem().green().bold());
    // println!("Private key: {}", key.priv_to_pem("").green().bold());

    // let res = key.sign(SIG_SAMPLE.to_string()).expect("Failed to sign");
    // println!("Signature: {}", res.green().bold());

    ////////////////////////////////

    // let message = "Hello, world!".to_string();
    // let crypted = key.crypt(&message).unwrap();
    // println!("Encrypted message: {}", crypted.green().bold());
    // let decrypted = key.decrypt(&crypted).unwrap();
    // println!("Decrypted message: {}", decrypted.green().bold());

    /////////////////////////////

    let priv_pem = "-----BEGIN PGP PRIVATE KEY BLOCK-----

xcMGBGgt2mYBCAC+qNpHFGOlzdblmmAh/0ckbmPRM4QU4puvWjojdstKnqk9mUfB
fCuTAlvRvwe5YrQs8TLKv5MlYxVY7kDw12Skt7jGGYpmvHA7xj9ywY/KPPT8p532
PmVdrSDu/JzSo5dpE096mMAgB4LP+FLxKPujxjmiROVlAYZvIfKqikTLQnAprrFG
BfIn10jU9M86GOmr/pRyhpiuupt2KWkNbxPIKj0H9QeLEfem202L4zA5Wfbj4d6E
NtI/YfEa8ndyIyaWSSFA5GNCG2NTdBXQIIUgLSqrOAT2OS0WZoHbKoxYF7d1MhIx
CqT9VZWur6GrvBWYv2PiTLrEeA0TBkg/8T2lABEBAAH+BwMIY7sPNzEBK1ZgSrxi
/cuF+Vi6IUp2hQj6QN/IaDBbPFM+kNIlgGF3RtvFD+jo/I10DxdjqlmvJYqmODZ/
2GrxzTXJgqMIee0+E1xsmtpzjx/9pzE3reoHWBCDIEyYCE6E3K4CgABz1KH+GGDT
Y8OJqdW1JwmMFeiwfL7dd7xcD4Zjxo6nV418xpZ8crFyXlK6Is95yDWPOI2+BgJ8
iXhoiK1WfHOn2365MvHqp5GbR/d8iQlbgr7yt9XcYu+oHm+aGNbk5MGTnQ+h+hY7
jlQYb/rbTIYzhDn7rpLi8K6IT+Q6JXs/zCHi1mNHtvxcV5f5Rr40Eu/O2biYfUVc
kXxRUgFYPzcu9p2lyzYXNuGodJI33Y/jKB1OsWMX/OYdX/eUgOSHYIeQ42EuEGGh
o4EME2puZ/xYKznHF6rZeCnnZN2pKd6wxQjblkrjNR0NtSxYcoCryBTW1KuAuSa5
s/zHiRbeO6RHKrrFKvBNGFKPsQrU9raAhTY9uUkV7WMT9X+y65TLyasgVmgDGz7O
Z264HktDdFzF/cZk84o1wfqZYwqdH2x6cZNFIyq72A4eAoOb1thmzB/7+zZizLOW
SwKuhtcXfVl5l6yLiih/qht8/DN0+DtnFeMPxP12tv+rfW6lXGquKt6+RsMnh1tP
QyzyrhIZNArkfauYAOtCCENvKsWIlQoCVgwQMpAb5Ai7G9EpNliq2sB1Q0MvY2yO
z5QtqTs96PckBpn7Xcc06t89oHY5Ln3fXrYt802JDx9rgPB0xihznPM6XfUk8Hg8
34pT1F/cGkBCSSo7florbqtdd8uOu7KH2GRIFDLZCWOpKWAEop59mCzEWwqbUmlM
NfnfF/aWjwNCO5rpuqe1HWe1dc7lRseLoWGZrO6S3lyFGMj6xuUScgmqyp0Gw7l+
953dyp1dNBvxzSx3aWxsaWFtLnZhbGVuZHVjIDx3aWxsaWFtLnZhbGVuZHVjQGVw
aXRhLmZyPsLAXAQTAQgAEAUCaC3aZgkQc6iG0jjCKH8AAH/tB/9C7BJ0njs7YKKD
a1d1hJtDWk2H09ZWKlQdx/tBwyGlGiMkQOFfnHeWIdfO9n+tATvo5FZw6JkVpfNX
/rLXoZ6Z8n8nWU4HN6v7n+wXd5TviGs7Ts0fdVzO8kDkFxMlMu2UGNA3PuhyQZhN
W6OMTryZBvzGSRmAY+jUtrYHGSfGofCG31xCMopd2z+fgVuPkBk1u2WTZmIxTHkU
lVXqj2YunGEZlQUxLTmNFs4vgwh4zvKPQA7uP0AzQYarlgzsmfEl8ha4DbmlrYvI
+EVuKC/9HdkzwqjUCSEoe9+zR+p7aL8K3mGCZ83W/xfpW56sbZImEp213RoSaR//
jzTbHisy
=T5dh
-----END PGP PRIVATE KEY BLOCK-----";

    //     // let crypted = "21886652079808314236336212688837582472269741600345807456695541031720769128463110680748835093784513606906593134179049457492027997637669706080519978470726969460034589424721868918672269094794915165558506457733288287923942221847448137188789626520482125781670512813445811984924212805855235444339331427791076590228992815113571495968049633091197178298435500338255690634212227299103858728117862465047442047679159218802387844580820785925171545139233220035947651018936306943172762567120173787107041220787767532398650689003298831111307972747804390506976805722430969368460205278577138700317836445344087003373195314416892528147610";
    //     // let crypted = "7099765350104687882003128230078064596903162510643154409112281759448863513182693994663951013812449022828067699543516048490296953218097251197965565494505056884039981488872934424803238710708797527883620470743536974795872955473328329383570264332063876702838742707305417608519505414738710093010564050371513821759677007230051442041924239941420119421961767210890659869966351378272662603464585410299781501584366047841263534302291353095997376285536551043701858283864686892575473702138504083550218066252744152331646931168182316110528366534032363531960455265437613793128204550667985245553156840738035028598558882518630605475164";
    //     let crypted = "10793081500379702944311231401604641417126670297459130674307682430311468988058025493585096615657839965709498000545958658542027905634057668998909084245997369516209175602027504321805106381395585404673555642477635352188504117536026268165733391572984093244465914468671842853403467738757663230005307452561902510566592867628894369168274344775451586636399163596398222313129253150081364483900205986337520922324370007237428235014033067622449101247457705486995412281312981441866095559149848336820069606528769191774300472581130710932046639254418268878191962648544675103100994985790741171500581246016679245880695064738483355788545";

    let key = KeyPair::priv_from_pem(priv_pem, "", &config.user_login).unwrap();
    //     let decrypted = key.decrypt(&crypted.to_string()).unwrap();
    //     println!("Decrypted message: {}", decrypted.green().bold());

    let tr = Transaction::new(
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        223,
        "william.valenduc",
        DataType::ClassicTransaction,
        key,
    )
    .expect("Failed to create transaction");

    // let a = String::from_utf8(&tr.signature)
    //     .expect(msg!("Failed to convert signature to string"));
    // println!("Signature: {}", a.green().bold());

    let json =
        serde_json::to_string(&tr).expect("Failed to serialize transaction");
    println!("Transaction: {}", json.green().bold());

    /////////////////////

    // let txt = "TRUC_A_SIGNER";
    // let sig = key.sign(txt.to_string()).unwrap();
    // // println!("sig1: {:?}", sig);
    // let b = sig.to_bytes_be().to_vec();
    // // println!("sig2: {:?} {}", b, b.len());

    // let c = sig.to_string();
    // println!("sig3: {:?}", c);
    // let d: BigUint = BigUint::from_bytes_be(&b);
    // let e = BigUint::from_str(c.as_str()).unwrap();

    // let res = key.check_signature(txt.to_string(), &e).unwrap();
    // println!("Signature valid: {}", res.to_string().green().bold());
}
