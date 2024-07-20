<?php
# load ini.php file
$inifile = __DIR__.'/wav_converter.ini.php';
$jsfile = __DIR__.'/wav_converter-worker.tpl.js';
if (file_exists($inifile)) {
    require_once $inifile;
} else {
    $soundfont = '../fonts/TimGM6mb.sf2';
    # $pkg_url = '../pkg';
    $pkg_url = "https://cdn.jsdelivr.net/npm/picosakura@0.1.33";
}
# output
header('Content-Type: application/x-javascript; charset=utf-8');
$js = file_get_contents($jsfile);
$js = str_replace('__SOUNDFONT__', $soundfont, $js);
$js = str_replace('__PKG_URL__', $pkg_url, $js);
echo $js;