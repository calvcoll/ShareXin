use VERSION;
use language;

static APP: &'static str = "sharexin ";

pub fn help() -> String {

    // snippets of help/usage message in 9 languages, will be formated
    let usage_fr = "Utilisation: ";
    let options_fr = "Options:";
    let help_fr = "Afficher le message d'aide et quitter";
    let version_fr = "Imprimer les informations de la version et quitter";
    let upgrade_fr = "Vérifiez les nouvelles mises à jour";
    let image_fr = "Options d'image:";
    let area_fr = "Capturer une zone de l'écran plutôt que l'écran complet";
    let window_fr = "Capturer la fenêtre active plutôt que l'écran complet";
    let full_fr = "Capturer l'écran complet";
    let file_fr = "Utiliser un fichier";
    let destinations_fr = "Destinations:";
    let toot_fr = "Upload vers Mastodon (en utilisant \"toot\")";
    let tweet_fr = "Upload vers Twitter (en utilisant \"t\")";
    let imgur_fr = "Upload vers Imgur";
    let examples_fr = "Exemples:";

    let usage_es = "Utilización: ";
    let options_es = "Opciones:";
    let help_es = "Mostrar el mensaje de ayuda y sale";
    let version_es = "Imprimir información de la versión y sale";
    let upgrade_es = "Busque nuevas actualizaciones";
    let image_es = "Opciones de imagen:";
    let area_es = "Capturar un área de la pantalla en lugar de la pantalla entera";
    let window_es = "Capturar una ventana en vez de la pantalla entera";
    let full_es = "Capturar la pantalla completa";
    let file_es = "Utilice un archive";
    let destinations_es = "Destinos:";
    let toot_es = "Sube a Mastodon (usando \"toot\")";
    let tweet_es = "Sube a Twitter (usando \"t\")";
    let imgur_es = "Sube a Imgur";
    let examples_es = "Ejemplos:";

    let usage_eo = "Uzo: ";
    let options_eo = "Opcioj:";
    let help_eo = "Montru la helpo mesaĝon kaj eliro";
    let version_eo = "Printversio informoj kaj eliro";
    let upgrade_eo = "Kontrolu por novaj ĝisdatigoj";
    let image_eo = "Opcioj de bildo:";
    let area_eo = "Kapti areon de la ekrano anstataŭ ol la tuta ekrano";
    let window_eo = "Kapti la aktivan fenestron anstataŭ ol la tuta ekrano";
    let full_eo = "Kapti la plena ekrano";
    let file_eo = "Uzu dosieron";
    let destinations_eo = "Celoj:";
    let toot_eo = "Alŝutu al Mastodon (uzante \"toot\")";
    let tweet_eo = "Alŝutu al Twitter (uzante \"t\")";
    let imgur_eo = "Alŝutu al Imgur";
    let examples_eo = "Ekzemploj:";

    let usage_cn = "使用方法: ";
    let options_cn = "选项:";
    let help_cn = "退出标准成功输出输出用法消息。";
    let version_cn = "在成功完成输出版本信息到标准输出。";
    let upgrade_cn = "要检查新的更新。";
    let image_cn = "选项截图:";
    let area_cn = "截取屏幕的一个区域，而不是整个屏幕";
    let window_cn = "截取窗口，而不是整个屏幕";
    let full_cn = "为了让整个屏幕";
    let file_cn = "使用文件";
    let destinations_cn = "目的地:";
    let toot_cn = "上传到Mastodon（使用 「toot」）";
    let tweet_cn = "上传到Twitter（使用 「t」）";
    let imgur_cn = "上传到Imgur";
    let examples_cn = "案件:";

    let usage_tw = "使用方法: ";
    let options_tw = "選項:";
    let help_tw = "退出標準成功輸出輸出用法消息。";
    let version_tw = "在成功完成輸出版本信息到標準輸出。";
    let upgrade_tw = "要檢查新的更新。";
    let image_tw = "選項截圖:";
    let area_tw = "擷取畫面的一個區域而不是整個畫面";
    let window_tw = "擷取單一視窗而不是整個畫面";
    let full_tw = "為了讓整個屏幕";
    let file_tw = "使用文件";
    let destinations_tw = "目的地:";
    let toot_tw = "上傳到Mastodon（使用 「toot」）";
    let tweet_tw = "上傳到Twitter（使用 「t」）";
    let imgur_tw = "上傳到Imgur";
    let examples_tw = "案件:";

    let usage_ja = "使用方法: ";
    let options_ja = "オプション:";
    let help_ja = "標準出力に使用方法のメッセージを出力して正常終了する。";
    let version_ja = "標準出力にバージョン情報を出力して正常終了する。";
    let upgrade_ja = "新しい更新を確認する。";
    let image_ja = "スクリーンショットのオプション:";
    let area_ja = "画面全体ではなく一部を取得する";
    let window_ja = "画面全体ではなくウィンドウ単体を取得する";
    let full_ja = "画面全体を取得する";
    let file_ja = "ファイルを使って";
    let destinations_ja = "行き先:";
    let toot_ja = "マストドンにアップロード（使用して「ｔｏｏｔ」)";
    let tweet_ja = "ツイッターにアップロード（使用して「ｔ」)";
    let imgur_ja = "Imgurにアップロード";
    let examples_ja = "例:";

    let usage_ko = "사용 방법: ";
    let options_ko = "옵션:";
    let help_ko = "표준 출력으로 사용법을 출력하고 정상적으로 종료한다.";
    let version_ko = "표준 출력으로 버전 정보를 출력하고 정상적으로 종료한다.";
    let upgrade_ko = "새로운 업데이트를 확인한다.";
    let image_ko = "스크린 샷 옵션:";
    let area_ko = "전체 화면 대신에 화면의 일정 영역을 찍습니다";
    let window_ko = "전체 화면 대신에 창을 찍습니다";
    let full_ko = "전체 화면을 얻을";
    let file_ko = "파일을 사용하여";
    let destinations_ko = "목적지:";
    let toot_ko = "Mastodon에 업로드 (사용 「toot」)";
    let tweet_ko = "Twitter에 업로드 (사용 「t」)";
    let imgur_ko = "Imgur에 업로드";
    let examples_ko = "예:";

    let usage_de = "Anwendung: ";
    let options_de = "Optionen:";
    let help_de = "Zeige diese Nachricht an und beende";
    let version_de = "Gebe Version aus und beende";
    let upgrade_de = "Auf neue Updates prüfen";
    let image_de = "Bildoptionen:";
    let area_de = "Nur einen Bereich anstatt des gesamten Bildschirms aufnehmen";
    let window_de = "Nur ein Fenster anstatt des gesamten Bildschirms aufnehmen";
    let full_de = "Gesamten Bildschirms aufnehmen";
    let file_de = "Benutze eine Datei";
    let destinations_de = "Reiseziele:";
    let toot_de = "Auf Mastodon veröffentlichen (benutzt \"toot\")";
    let tweet_de = "Auf Twitter veröffentlichen (benutzt \"t\")";
    let imgur_de = "Auf Imgur veröffentlichen";
    let examples_de = "Beispiele:";

    let usage_en = "Usage: ";
    let options_en = "Options:";
    let help_en = "Display this help message and exit";
    let version_en = "Print version info and exit";
    let upgrade_en = "Check for new updates";
    let image_en = "Image Options:";
    let area_en = "Grab an area of the screen instead of the entire screen";
    let window_en = "Grab the current window instead of the entire screen";
    let full_en = "Grab the entire screen";
    let file_en = "Use a file";
    let destinations_en = "Destinations:";
    let toot_en = "Upload to Mastodon (uses \"toot\")";
    let tweet_en = "Upload to Twitter (uses \"t\")";
    let imgur_en = "Upload to Imgur";
    let examples_en = "Examples:";

    let usage_usage = "sharexin <options> [destination] [image options] [FILE]";
    let usage_examples = "  sharexin toot
  sharexin tweet full
  sharexin toot area
  sharexin imgur file [FILE]";

    // templates
    let usage: &str;
    let options: &str;
    let help: &str;
    let version: &str;
    let upgrade: &str;
    let image: &str;
    let area: &str;
    let window: &str;
    let full: &str;
    let file: &str;
    let destinations: &str;
    let toot: &str;
    let tweet: &str;
    let imgur: &str;
    let examples: &str;

    let _lang = language::locale();
    let lang = &_lang.to_lowercase();

    if lang.contains("fr") {
        usage = usage_fr;
        options = options_fr;
        help = help_fr;
        version = version_fr;
        upgrade = upgrade_fr;
        image = image_fr;
        area = area_fr;
        window = window_fr;
        full = full_fr;
        file = file_fr;
        destinations = destinations_fr;
        toot = toot_fr;
        tweet = tweet_fr;
        imgur = imgur_fr;
        examples = examples_fr;
    } else if lang.contains("es") {
        usage = usage_es;
        options = options_es;
        help = help_es;
        version = version_es;
        upgrade = upgrade_es;
        image = image_es;
        area = area_es;
        window = window_es;
        full = full_es;
        file = file_es;
        destinations = destinations_es;
        toot = toot_es;
        tweet = tweet_es;
        imgur = imgur_es;
        examples = examples_es;
    } else if lang.contains("eo") {
        usage = usage_eo;
        options = options_eo;
        help = help_eo;
        version = version_eo;
        upgrade = upgrade_eo;
        image = image_eo;
        area = area_eo;
        window = window_eo;
        full = full_eo;
        file = file_eo;
        destinations = destinations_eo;
        toot = toot_eo;
        tweet = tweet_eo;
        imgur = imgur_eo;
        examples = examples_eo;
    } else if lang.contains("cn") {
        usage = usage_cn;
        options = options_cn;
        help = help_cn;
        version = version_cn;
        upgrade = upgrade_cn;
        image = image_cn;
        area = area_cn;
        window = window_cn;
        full = full_cn;
        file = file_cn;
        destinations = destinations_cn;
        toot = toot_cn;
        tweet = tweet_cn;
        imgur = imgur_cn;
        examples = examples_cn;
    } else if lang.contains("tw") {
        usage = usage_tw;
        options = options_tw;
        help = help_tw;
        version = version_tw;
        upgrade = upgrade_tw;
        image = image_tw;
        area = area_tw;
        window = window_tw;
        full = full_tw;
        file = file_tw;
        destinations = destinations_tw;
        toot = toot_tw;
        tweet = tweet_tw;
        imgur = imgur_tw;
        examples = examples_tw;
    } else if lang.contains("ja") {
        usage = usage_ja;
        options = options_ja;
        help = help_ja;
        version = version_ja;
        upgrade = upgrade_ja;
        image = image_ja;
        area = area_ja;
        window = window_ja;
        full = full_ja;
        file = file_ja;
        destinations = destinations_ja;
        toot = toot_ja;
        tweet = tweet_ja;
        imgur = imgur_ja;
        examples = examples_ja;
    } else if lang.contains("ko") {
        usage = usage_ko;
        options = options_ko;
        help = help_ko;
        version = version_ko;
        upgrade = upgrade_ko;
        image = image_ko;
        area = area_ko;
        window = window_ko;
        full = full_ko;
        file = file_ko;
        destinations = destinations_ko;
        toot = toot_ko;
        tweet = tweet_ko;
        imgur = imgur_ko;
        examples = examples_ko;
    } else if lang.contains("de") {
        usage = usage_de;
        options = options_de;
        help = help_de;
        version = version_de;
        upgrade = upgrade_de;
        image = image_de;
        area = area_de;
        window = window_de;
        full = full_de;
        file = file_de;
        destinations = destinations_de;
        toot = toot_de;
        tweet = tweet_de;
        imgur = imgur_de;
        examples = examples_de;
    } else {
        usage = usage_en;
        options = options_en;
        help = help_en;
        version = version_en;
        upgrade = upgrade_en;
        image = image_en;
        area = area_en;
        window = window_en;
        full = full_en;
        file = file_en;
        destinations = destinations_en;
        toot = toot_en;
        tweet = tweet_en;
        imgur = imgur_en;
        examples = examples_en;
    }

    return format!(
        "{}{}\n{}{}\n\n{}\n  -h, --help\t{}\n  -V, --version\t{}\n  -U, --upgrade\t{}\n
{}\n  area\t\t{}\n  window\t{}\n  full\t\t{}\n  file\t\t{}\n
{}\n  toot\t\t{}\n  tweet\t\t{}\n  imgur\t\t{}\n
{}\n{}",
        APP,
        VERSION,
        usage,
        usage_usage,
        options,
        help,
        version,
        upgrade,
        image,
        area,
        window,
        full,
        file,
        destinations,
        toot,
        tweet,
        imgur,
        examples,
        usage_examples
    );

}
