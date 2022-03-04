use super::Kanji;
#[allow(dead_code)]
pub const KANJIS: [Kanji;1026] = [
	Kanji {
		id: 0,
		kanji: '一',
		strokes: 1,
		meanings: &["one"],
		on_yomis: &["ichi","itsu"],
		kun_yomis: &["hito-tsu"]
	},
	Kanji {
		id: 1,
		kanji: '二',
		strokes: 2,
		meanings: &["two"],
		on_yomis: &["ni","ji"],
		kun_yomis: &["futa-tsu"]
	},
	Kanji {
		id: 2,
		kanji: '三',
		strokes: 3,
		meanings: &["three"],
		on_yomis: &["san"],
		kun_yomis: &["mit-tsu"]
	},
	Kanji {
		id: 3,
		kanji: '四',
		strokes: 5,
		meanings: &["four"],
		on_yomis: &["shi"],
		kun_yomis: &["yot-tsu","yon"]
	},
	Kanji {
		id: 4,
		kanji: '五',
		strokes: 4,
		meanings: &["five"],
		on_yomis: &["go"],
		kun_yomis: &["itsu-tsu"]
	},
	Kanji {
		id: 5,
		kanji: '六',
		strokes: 4,
		meanings: &["six"],
		on_yomis: &["roku"],
		kun_yomis: &["mut-tsu"]
	},
	Kanji {
		id: 6,
		kanji: '七',
		strokes: 2,
		meanings: &["seven"],
		on_yomis: &["shichi"],
		kun_yomis: &["nana-tsu","nana"]
	},
	Kanji {
		id: 7,
		kanji: '八',
		strokes: 2,
		meanings: &["eight"],
		on_yomis: &["hachi"],
		kun_yomis: &["yat-tsu"]
	},
	Kanji {
		id: 8,
		kanji: '九',
		strokes: 2,
		meanings: &["nine"],
		on_yomis: &["ku","kyū"],
		kun_yomis: &["kokono-tsu"]
	},
	Kanji {
		id: 9,
		kanji: '十',
		strokes: 2,
		meanings: &["ten"],
		on_yomis: &["jū"],
		kun_yomis: &["tō"]
	},
	Kanji {
		id: 10,
		kanji: '百',
		strokes: 6,
		meanings: &["hundred"],
		on_yomis: &["hyaku"],
		kun_yomis: &["momo"]
	},
	Kanji {
		id: 11,
		kanji: '千',
		strokes: 3,
		meanings: &["thousand"],
		on_yomis: &["sen"],
		kun_yomis: &["chi"]
	},
	Kanji {
		id: 12,
		kanji: '上',
		strokes: 3,
		meanings: &["top","above"],
		on_yomis: &["jō"],
		kun_yomis: &["ue"]
	},
	Kanji {
		id: 13,
		kanji: '下',
		strokes: 3,
		meanings: &["bottom","below"],
		on_yomis: &["ka","ge"],
		kun_yomis: &["shita","shimo","moto"]
	},
	Kanji {
		id: 14,
		kanji: '左',
		strokes: 5,
		meanings: &["left"],
		on_yomis: &["sa"],
		kun_yomis: &["hidari"]
	},
	Kanji {
		id: 15,
		kanji: '右',
		strokes: 5,
		meanings: &["right"],
		on_yomis: &["u","yū"],
		kun_yomis: &["migi"]
	},
	Kanji {
		id: 16,
		kanji: '中',
		strokes: 4,
		meanings: &["inside","middle"],
		on_yomis: &["chū","jū"],
		kun_yomis: &["naka"]
	},
	Kanji {
		id: 17,
		kanji: '大',
		strokes: 3,
		meanings: &["large"],
		on_yomis: &["dai","tai"],
		kun_yomis: &["ō-kii","ō"]
	},
	Kanji {
		id: 18,
		kanji: '小',
		strokes: 3,
		meanings: &["small"],
		on_yomis: &["shō"],
		kun_yomis: &["chii-sai","ko","o"]
	},
	Kanji {
		id: 19,
		kanji: '月',
		strokes: 4,
		meanings: &["month","moon"],
		on_yomis: &["gatsu","getsu"],
		kun_yomis: &["tsuki"]
	},
	Kanji {
		id: 20,
		kanji: '日',
		strokes: 4,
		meanings: &["day","sun"],
		on_yomis: &["nichi","jitsu"],
		kun_yomis: &["hi","ka"]
	},
	Kanji {
		id: 21,
		kanji: '年',
		strokes: 6,
		meanings: &["year"],
		on_yomis: &["nen"],
		kun_yomis: &["toshi"]
	},
	Kanji {
		id: 22,
		kanji: '早',
		strokes: 6,
		meanings: &["early"],
		on_yomis: &["sō","sa"],
		kun_yomis: &["haya-i"]
	},
	Kanji {
		id: 23,
		kanji: '木',
		strokes: 4,
		meanings: &["tree"],
		on_yomis: &["moku","boku"],
		kun_yomis: &["ki"]
	},
	Kanji {
		id: 24,
		kanji: '林',
		strokes: 8,
		meanings: &["woods"],
		on_yomis: &["rin"],
		kun_yomis: &["hayashi"]
	},
	Kanji {
		id: 25,
		kanji: '山',
		strokes: 3,
		meanings: &["mountain"],
		on_yomis: &["san"],
		kun_yomis: &["yama"]
	},
	Kanji {
		id: 26,
		kanji: '川',
		strokes: 3,
		meanings: &["river"],
		on_yomis: &["sen"],
		kun_yomis: &["kawa"]
	},
	Kanji {
		id: 27,
		kanji: '土',
		strokes: 3,
		meanings: &["soil"],
		on_yomis: &["to","do"],
		kun_yomis: &["tsuchi"]
	},
	Kanji {
		id: 28,
		kanji: '空',
		strokes: 8,
		meanings: &["sky"],
		on_yomis: &["kū"],
		kun_yomis: &["sora","a-ku","kara"]
	},
	Kanji {
		id: 29,
		kanji: '田',
		strokes: 5,
		meanings: &["rice field"],
		on_yomis: &["den"],
		kun_yomis: &["da","ta"]
	},
	Kanji {
		id: 30,
		kanji: '天',
		strokes: 4,
		meanings: &["heaven","sky"],
		on_yomis: &["ten"],
		kun_yomis: &["ame","ama"]
	},
	Kanji {
		id: 31,
		kanji: '生',
		strokes: 5,
		meanings: &["living","birth","raw"],
		on_yomis: &["sei","shō"],
		kun_yomis: &["i-kiru","u-mu","nama"]
	},
	Kanji {
		id: 32,
		kanji: '花',
		strokes: 7,
		meanings: &["flower"],
		on_yomis: &["ka"],
		kun_yomis: &["hana"]
	},
	Kanji {
		id: 33,
		kanji: '草',
		strokes: 9,
		meanings: &["grass"],
		on_yomis: &["sō"],
		kun_yomis: &["kusa"]
	},
	Kanji {
		id: 34,
		kanji: '虫',
		strokes: 6,
		meanings: &["insect"],
		on_yomis: &["chū"],
		kun_yomis: &["mushi"]
	},
	Kanji {
		id: 35,
		kanji: '犬',
		strokes: 4,
		meanings: &["dog"],
		on_yomis: &["ken"],
		kun_yomis: &["inu"]
	},
	Kanji {
		id: 36,
		kanji: '人',
		strokes: 2,
		meanings: &["person"],
		on_yomis: &["jin","nin"],
		kun_yomis: &["hito"]
	},
	Kanji {
		id: 37,
		kanji: '名',
		strokes: 6,
		meanings: &["name"],
		on_yomis: &["mei","myō"],
		kun_yomis: &["na"]
	},
	Kanji {
		id: 38,
		kanji: '女',
		strokes: 3,
		meanings: &["female"],
		on_yomis: &["jo","nyo"],
		kun_yomis: &["on'na"]
	},
	Kanji {
		id: 39,
		kanji: '男',
		strokes: 7,
		meanings: &["male"],
		on_yomis: &["dan","nan"],
		kun_yomis: &["otoko"]
	},
	Kanji {
		id: 40,
		kanji: '子',
		strokes: 3,
		meanings: &["child"],
		on_yomis: &["shi","su"],
		kun_yomis: &["ko"]
	},
	Kanji {
		id: 41,
		kanji: '目',
		strokes: 5,
		meanings: &["eye"],
		on_yomis: &["moku"],
		kun_yomis: &["me"]
	},
	Kanji {
		id: 42,
		kanji: '耳',
		strokes: 6,
		meanings: &["ear"],
		on_yomis: &["ji","ni"],
		kun_yomis: &["mimi"]
	},
	Kanji {
		id: 43,
		kanji: '口',
		strokes: 3,
		meanings: &["mouth"],
		on_yomis: &["kō"],
		kun_yomis: &["kuchi"]
	},
	Kanji {
		id: 44,
		kanji: '手',
		strokes: 4,
		meanings: &["hand"],
		on_yomis: &["shu"],
		kun_yomis: &["te"]
	},
	Kanji {
		id: 45,
		kanji: '足',
		strokes: 7,
		meanings: &["foot","suffice"],
		on_yomis: &["soku"],
		kun_yomis: &["ashi","ta-riru"]
	},
	Kanji {
		id: 46,
		kanji: '見',
		strokes: 7,
		meanings: &["see"],
		on_yomis: &["ken","gen"],
		kun_yomis: &["mi-ru"]
	},
	Kanji {
		id: 47,
		kanji: '音',
		strokes: 9,
		meanings: &["sound"],
		on_yomis: &["on"],
		kun_yomis: &["ne","oto"]
	},
	Kanji {
		id: 48,
		kanji: '力',
		strokes: 2,
		meanings: &["power"],
		on_yomis: &["riki","ryoku"],
		kun_yomis: &["chikara"]
	},
	Kanji {
		id: 49,
		kanji: '気',
		strokes: 6,
		meanings: &["spirit","air"],
		on_yomis: &["ki","ke"],
		kun_yomis: &["iki"]
	},
	Kanji {
		id: 50,
		kanji: '円',
		strokes: 4,
		meanings: &["yen","circle"],
		on_yomis: &["en"],
		kun_yomis: &["maru"]
	},
	Kanji {
		id: 51,
		kanji: '入',
		strokes: 2,
		meanings: &["enter"],
		on_yomis: &["nyū"],
		kun_yomis: &["hai-ru","i-ru"]
	},
	Kanji {
		id: 52,
		kanji: '出',
		strokes: 5,
		meanings: &["exit"],
		on_yomis: &["shutsu"],
		kun_yomis: &["de-ru"]
	},
	Kanji {
		id: 53,
		kanji: '立',
		strokes: 5,
		meanings: &["stand up"],
		on_yomis: &["ritsu"],
		kun_yomis: &["ta-tsu"]
	},
	Kanji {
		id: 54,
		kanji: '休',
		strokes: 6,
		meanings: &["rest"],
		on_yomis: &["kyū"],
		kun_yomis: &["yasu-mu"]
	},
	Kanji {
		id: 55,
		kanji: '先',
		strokes: 6,
		meanings: &["previous"],
		on_yomis: &["sen"],
		kun_yomis: &["saki"]
	},
	Kanji {
		id: 56,
		kanji: '夕',
		strokes: 3,
		meanings: &["evening"],
		on_yomis: &["seki"],
		kun_yomis: &["yū"]
	},
	Kanji {
		id: 57,
		kanji: '本',
		strokes: 5,
		meanings: &["book"],
		on_yomis: &["hon"],
		kun_yomis: &["moto"]
	},
	Kanji {
		id: 58,
		kanji: '文',
		strokes: 4,
		meanings: &["text"],
		on_yomis: &["bun","mon"],
		kun_yomis: &["fumi"]
	},
	Kanji {
		id: 59,
		kanji: '字',
		strokes: 6,
		meanings: &["character"],
		on_yomis: &["ji"],
		kun_yomis: &["aza"]
	},
	Kanji {
		id: 60,
		kanji: '学',
		strokes: 8,
		meanings: &["study"],
		on_yomis: &["gaku"],
		kun_yomis: &["mana-bu"]
	},
	Kanji {
		id: 61,
		kanji: '校',
		strokes: 10,
		meanings: &["school"],
		on_yomis: &["kō"],
		kun_yomis: &["kase"]
	},
	Kanji {
		id: 62,
		kanji: '村',
		strokes: 7,
		meanings: &["village"],
		on_yomis: &["son"],
		kun_yomis: &["mura"]
	},
	Kanji {
		id: 63,
		kanji: '町',
		strokes: 7,
		meanings: &["town"],
		on_yomis: &["chō"],
		kun_yomis: &["machi"]
	},
	Kanji {
		id: 64,
		kanji: '森',
		strokes: 12,
		meanings: &["forest"],
		on_yomis: &["shin"],
		kun_yomis: &["mori"]
	},
	Kanji {
		id: 65,
		kanji: '正',
		strokes: 5,
		meanings: &["correct"],
		on_yomis: &["sei","shō"],
		kun_yomis: &["tada-shii","masa"]
	},
	Kanji {
		id: 66,
		kanji: '水',
		strokes: 4,
		meanings: &["water"],
		on_yomis: &["sui"],
		kun_yomis: &["mizu"]
	},
	Kanji {
		id: 67,
		kanji: '火',
		strokes: 4,
		meanings: &["fire"],
		on_yomis: &["ka"],
		kun_yomis: &["hi"]
	},
	Kanji {
		id: 68,
		kanji: '玉',
		strokes: 5,
		meanings: &["ball"],
		on_yomis: &["gyoku"],
		kun_yomis: &["tama"]
	},
	Kanji {
		id: 69,
		kanji: '王',
		strokes: 4,
		meanings: &["king"],
		on_yomis: &["ō"],
		kun_yomis: &["kimi"]
	},
	Kanji {
		id: 70,
		kanji: '石',
		strokes: 5,
		meanings: &["stone"],
		on_yomis: &["seki","koku"],
		kun_yomis: &["ishi"]
	},
	Kanji {
		id: 71,
		kanji: '竹',
		strokes: 6,
		meanings: &["bamboo"],
		on_yomis: &["chiku"],
		kun_yomis: &["take"]
	},
	Kanji {
		id: 72,
		kanji: '糸',
		strokes: 6,
		meanings: &["thread"],
		on_yomis: &["shi"],
		kun_yomis: &["ito"]
	},
	Kanji {
		id: 73,
		kanji: '貝',
		strokes: 7,
		meanings: &["shellfish"],
		on_yomis: &["bai"],
		kun_yomis: &["kai"]
	},
	Kanji {
		id: 74,
		kanji: '車',
		strokes: 7,
		meanings: &["vehicle"],
		on_yomis: &["sha"],
		kun_yomis: &["kuruma"]
	},
	Kanji {
		id: 75,
		kanji: '金',
		strokes: 8,
		meanings: &["gold","money"],
		on_yomis: &["kin"],
		kun_yomis: &["kane","kana"]
	},
	Kanji {
		id: 76,
		kanji: '雨',
		strokes: 8,
		meanings: &["rain"],
		on_yomis: &["u"],
		kun_yomis: &["ame","ama"]
	},
	Kanji {
		id: 77,
		kanji: '赤',
		strokes: 7,
		meanings: &["red"],
		on_yomis: &["seki"],
		kun_yomis: &["aka"]
	},
	Kanji {
		id: 78,
		kanji: '青',
		strokes: 8,
		meanings: &["blue"],
		on_yomis: &["sei","shō"],
		kun_yomis: &["ao"]
	},
	Kanji {
		id: 79,
		kanji: '白',
		strokes: 5,
		meanings: &["white"],
		on_yomis: &["haku"],
		kun_yomis: &["shiro","shira"]
	},
	Kanji {
		id: 80,
		kanji: '数',
		strokes: 13,
		meanings: &["number","count"],
		on_yomis: &["sū"],
		kun_yomis: &["kazu"]
	},
	Kanji {
		id: 81,
		kanji: '多',
		strokes: 6,
		meanings: &["many","much"],
		on_yomis: &["ta"],
		kun_yomis: &["oo-i"]
	},
	Kanji {
		id: 82,
		kanji: '少',
		strokes: 4,
		meanings: &["a few","a little"],
		on_yomis: &["shō"],
		kun_yomis: &["suku-nai","suko-shi"]
	},
	Kanji {
		id: 83,
		kanji: '万',
		strokes: 3,
		meanings: &["ten thousand"],
		on_yomis: &["ban","man"],
		kun_yomis: &["yorozu"]
	},
	Kanji {
		id: 84,
		kanji: '半',
		strokes: 5,
		meanings: &["half"],
		on_yomis: &["han"],
		kun_yomis: &["naka-ba"]
	},
	Kanji {
		id: 85,
		kanji: '形',
		strokes: 7,
		meanings: &["shape"],
		on_yomis: &["kei","gyō"],
		kun_yomis: &["katachi"]
	},
	Kanji {
		id: 86,
		kanji: '太',
		strokes: 4,
		meanings: &["thick"],
		on_yomis: &["ta"],
		kun_yomis: &["futo-i"]
	},
	Kanji {
		id: 87,
		kanji: '細',
		strokes: 11,
		meanings: &["thin"],
		on_yomis: &["sai"],
		kun_yomis: &["hoso-i"]
	},
	Kanji {
		id: 88,
		kanji: '広',
		strokes: 5,
		meanings: &["wide"],
		on_yomis: &["kō"],
		kun_yomis: &["hiro-i"]
	},
	Kanji {
		id: 89,
		kanji: '長',
		strokes: 8,
		meanings: &["long","leader"],
		on_yomis: &["chō"],
		kun_yomis: &["naga-i"]
	},
	Kanji {
		id: 90,
		kanji: '点',
		strokes: 9,
		meanings: &["point"],
		on_yomis: &["ten"],
		kun_yomis: &["bochi"]
	},
	Kanji {
		id: 91,
		kanji: '丸',
		strokes: 3,
		meanings: &["circle"],
		on_yomis: &["gan"],
		kun_yomis: &["maru"]
	},
	Kanji {
		id: 92,
		kanji: '交',
		strokes: 6,
		meanings: &["intersect"],
		on_yomis: &["kō"],
		kun_yomis: &["maji-waru"]
	},
	Kanji {
		id: 93,
		kanji: '光',
		strokes: 6,
		meanings: &["light"],
		on_yomis: &["kō"],
		kun_yomis: &["hikari"]
	},
	Kanji {
		id: 94,
		kanji: '角',
		strokes: 7,
		meanings: &["corner","horn"],
		on_yomis: &["kaku"],
		kun_yomis: &["kado","tsuno","sumi"]
	},
	Kanji {
		id: 95,
		kanji: '計',
		strokes: 9,
		meanings: &["measure"],
		on_yomis: &["kei"],
		kun_yomis: &["haka-ru"]
	},
	Kanji {
		id: 96,
		kanji: '直',
		strokes: 8,
		meanings: &["straight","fix"],
		on_yomis: &["choku","jiki"],
		kun_yomis: &["tada-chi","nao-su"]
	},
	Kanji {
		id: 97,
		kanji: '線',
		strokes: 15,
		meanings: &["line"],
		on_yomis: &["sen"],
		kun_yomis: &["suji"]
	},
	Kanji {
		id: 98,
		kanji: '矢',
		strokes: 5,
		meanings: &["arrow"],
		on_yomis: &["shi"],
		kun_yomis: &["ya"]
	},
	Kanji {
		id: 99,
		kanji: '弱',
		strokes: 10,
		meanings: &["weak"],
		on_yomis: &["jaku"],
		kun_yomis: &["yowa-i"]
	},
	Kanji {
		id: 100,
		kanji: '強',
		strokes: 11,
		meanings: &["strong"],
		on_yomis: &["kyō"],
		kun_yomis: &["tsuyo-i"]
	},
	Kanji {
		id: 101,
		kanji: '高',
		strokes: 10,
		meanings: &["high"],
		on_yomis: &["kō"],
		kun_yomis: &["taka-i"]
	},
	Kanji {
		id: 102,
		kanji: '同',
		strokes: 6,
		meanings: &["same"],
		on_yomis: &["dō"],
		kun_yomis: &["ona-ji"]
	},
	Kanji {
		id: 103,
		kanji: '親',
		strokes: 16,
		meanings: &["parent"],
		on_yomis: &["shin"],
		kun_yomis: &["oya"]
	},
	Kanji {
		id: 104,
		kanji: '母',
		strokes: 5,
		meanings: &["mother"],
		on_yomis: &["bo"],
		kun_yomis: &["haha","kā"]
	},
	Kanji {
		id: 105,
		kanji: '父',
		strokes: 4,
		meanings: &["father"],
		on_yomis: &["fu"],
		kun_yomis: &["chichi","tou"]
	},
	Kanji {
		id: 106,
		kanji: '姉',
		strokes: 8,
		meanings: &["older sister"],
		on_yomis: &["shi"],
		kun_yomis: &["ane"]
	},
	Kanji {
		id: 107,
		kanji: '兄',
		strokes: 5,
		meanings: &["older brother"],
		on_yomis: &["kei","kyō"],
		kun_yomis: &["ani"]
	},
	Kanji {
		id: 108,
		kanji: '弟',
		strokes: 7,
		meanings: &["younger brother"],
		on_yomis: &["tei","dai"],
		kun_yomis: &["otōto"]
	},
	Kanji {
		id: 109,
		kanji: '妹',
		strokes: 8,
		meanings: &["younger sister"],
		on_yomis: &["mai"],
		kun_yomis: &["imōto"]
	},
	Kanji {
		id: 110,
		kanji: '自',
		strokes: 6,
		meanings: &["oneself"],
		on_yomis: &["ji","shi"],
		kun_yomis: &["mizuka-ra"]
	},
	Kanji {
		id: 111,
		kanji: '友',
		strokes: 4,
		meanings: &["friend"],
		on_yomis: &["yū"],
		kun_yomis: &["tomo"]
	},
	Kanji {
		id: 112,
		kanji: '体',
		strokes: 7,
		meanings: &["body"],
		on_yomis: &["tai"],
		kun_yomis: &["karada"]
	},
	Kanji {
		id: 113,
		kanji: '毛',
		strokes: 4,
		meanings: &["hair"],
		on_yomis: &["mō"],
		kun_yomis: &["ke"]
	},
	Kanji {
		id: 114,
		kanji: '頭',
		strokes: 16,
		meanings: &["head"],
		on_yomis: &["tō"],
		kun_yomis: &["atama"]
	},
	Kanji {
		id: 115,
		kanji: '顔',
		strokes: 18,
		meanings: &["face"],
		on_yomis: &["gan"],
		kun_yomis: &["kao"]
	},
	Kanji {
		id: 116,
		kanji: '首',
		strokes: 9,
		meanings: &["neck"],
		on_yomis: &["shu"],
		kun_yomis: &["kubi"]
	},
	Kanji {
		id: 117,
		kanji: '心',
		strokes: 4,
		meanings: &["heart"],
		on_yomis: &["shin"],
		kun_yomis: &["kokoro"]
	},
	Kanji {
		id: 118,
		kanji: '時',
		strokes: 10,
		meanings: &["time"],
		on_yomis: &["ji"],
		kun_yomis: &["toki"]
	},
	Kanji {
		id: 119,
		kanji: '曜',
		strokes: 18,
		meanings: &["day of the week"],
		on_yomis: &["yō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 120,
		kanji: '朝',
		strokes: 12,
		meanings: &["morning"],
		on_yomis: &["chō"],
		kun_yomis: &["asa"]
	},
	Kanji {
		id: 121,
		kanji: '昼',
		strokes: 9,
		meanings: &["daytime"],
		on_yomis: &["chū"],
		kun_yomis: &["hiru"]
	},
	Kanji {
		id: 122,
		kanji: '夜',
		strokes: 8,
		meanings: &["night"],
		on_yomis: &["ya"],
		kun_yomis: &["yoru"]
	},
	Kanji {
		id: 123,
		kanji: '分',
		strokes: 4,
		meanings: &["minute","understand"],
		on_yomis: &["fun","bun"],
		kun_yomis: &["wa-karu"]
	},
	Kanji {
		id: 124,
		kanji: '週',
		strokes: 11,
		meanings: &["week"],
		on_yomis: &["shū"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 125,
		kanji: '春',
		strokes: 9,
		meanings: &["spring"],
		on_yomis: &["shun"],
		kun_yomis: &["haru"]
	},
	Kanji {
		id: 126,
		kanji: '夏',
		strokes: 10,
		meanings: &["summer"],
		on_yomis: &["ka"],
		kun_yomis: &["natsu"]
	},
	Kanji {
		id: 127,
		kanji: '秋',
		strokes: 9,
		meanings: &["autumn"],
		on_yomis: &["shū"],
		kun_yomis: &["aki"]
	},
	Kanji {
		id: 128,
		kanji: '冬',
		strokes: 5,
		meanings: &["winter"],
		on_yomis: &["tō"],
		kun_yomis: &["fuyu"]
	},
	Kanji {
		id: 129,
		kanji: '今',
		strokes: 4,
		meanings: &["now"],
		on_yomis: &["kon"],
		kun_yomis: &["ima"]
	},
	Kanji {
		id: 130,
		kanji: '新',
		strokes: 13,
		meanings: &["new"],
		on_yomis: &["shin"],
		kun_yomis: &["atara-shii","ara-ta"]
	},
	Kanji {
		id: 131,
		kanji: '古',
		strokes: 5,
		meanings: &["old"],
		on_yomis: &["ko"],
		kun_yomis: &["furu-i"]
	},
	Kanji {
		id: 132,
		kanji: '間',
		strokes: 12,
		meanings: &["interval"],
		on_yomis: &["kan","ken"],
		kun_yomis: &["ma","aida"]
	},
	Kanji {
		id: 133,
		kanji: '方',
		strokes: 4,
		meanings: &["direction"],
		on_yomis: &["hō"],
		kun_yomis: &["kata"]
	},
	Kanji {
		id: 134,
		kanji: '北',
		strokes: 5,
		meanings: &["north"],
		on_yomis: &["hoku"],
		kun_yomis: &["kita"]
	},
	Kanji {
		id: 135,
		kanji: '南',
		strokes: 9,
		meanings: &["south"],
		on_yomis: &["nan"],
		kun_yomis: &["minami"]
	},
	Kanji {
		id: 136,
		kanji: '東',
		strokes: 8,
		meanings: &["east"],
		on_yomis: &["tō"],
		kun_yomis: &["higashi","azuma"]
	},
	Kanji {
		id: 137,
		kanji: '西',
		strokes: 6,
		meanings: &["west"],
		on_yomis: &["sei","sai"],
		kun_yomis: &["nishi"]
	},
	Kanji {
		id: 138,
		kanji: '遠',
		strokes: 13,
		meanings: &["far"],
		on_yomis: &["en"],
		kun_yomis: &["tō-i"]
	},
	Kanji {
		id: 139,
		kanji: '近',
		strokes: 7,
		meanings: &["near"],
		on_yomis: &["kin"],
		kun_yomis: &["chika-i"]
	},
	Kanji {
		id: 140,
		kanji: '前',
		strokes: 9,
		meanings: &["before"],
		on_yomis: &["zen"],
		kun_yomis: &["mae"]
	},
	Kanji {
		id: 141,
		kanji: '後',
		strokes: 9,
		meanings: &["after"],
		on_yomis: &["go","kō"],
		kun_yomis: &["nochi","ushi-ro","ato"]
	},
	Kanji {
		id: 142,
		kanji: '内',
		strokes: 4,
		meanings: &["inside"],
		on_yomis: &["nai"],
		kun_yomis: &["uchi"]
	},
	Kanji {
		id: 143,
		kanji: '外',
		strokes: 5,
		meanings: &["outside"],
		on_yomis: &["gai","ge"],
		kun_yomis: &["soto","hoka","hazu-su"]
	},
	Kanji {
		id: 144,
		kanji: '場',
		strokes: 12,
		meanings: &["place"],
		on_yomis: &["jō"],
		kun_yomis: &["ba"]
	},
	Kanji {
		id: 145,
		kanji: '地',
		strokes: 6,
		meanings: &["ground"],
		on_yomis: &["chi","ji"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 146,
		kanji: '国',
		strokes: 8,
		meanings: &["country"],
		on_yomis: &["koku"],
		kun_yomis: &["kuni"]
	},
	Kanji {
		id: 147,
		kanji: '園',
		strokes: 13,
		meanings: &["garden"],
		on_yomis: &["en"],
		kun_yomis: &["sono"]
	},
	Kanji {
		id: 148,
		kanji: '谷',
		strokes: 7,
		meanings: &["valley"],
		on_yomis: &["koku"],
		kun_yomis: &["tani"]
	},
	Kanji {
		id: 149,
		kanji: '野',
		strokes: 11,
		meanings: &["field"],
		on_yomis: &["ya"],
		kun_yomis: &["no"]
	},
	Kanji {
		id: 150,
		kanji: '原',
		strokes: 10,
		meanings: &["meadow","plain"],
		on_yomis: &["gen"],
		kun_yomis: &["hara"]
	},
	Kanji {
		id: 151,
		kanji: '里',
		strokes: 7,
		meanings: &["hometown"],
		on_yomis: &["ri"],
		kun_yomis: &["sato"]
	},
	Kanji {
		id: 152,
		kanji: '市',
		strokes: 5,
		meanings: &["city"],
		on_yomis: &["shi"],
		kun_yomis: &["ichi"]
	},
	Kanji {
		id: 153,
		kanji: '京',
		strokes: 8,
		meanings: &["capital"],
		on_yomis: &["kyō","kei"],
		kun_yomis: &["miyako"]
	},
	Kanji {
		id: 154,
		kanji: '風',
		strokes: 9,
		meanings: &["wind","-style"],
		on_yomis: &["fū"],
		kun_yomis: &["kaze"]
	},
	Kanji {
		id: 155,
		kanji: '雪',
		strokes: 11,
		meanings: &["snow"],
		on_yomis: &["setsu"],
		kun_yomis: &["yuki"]
	},
	Kanji {
		id: 156,
		kanji: '雲',
		strokes: 12,
		meanings: &["cloud"],
		on_yomis: &["un"],
		kun_yomis: &["kumo"]
	},
	Kanji {
		id: 157,
		kanji: '池',
		strokes: 6,
		meanings: &["pond"],
		on_yomis: &["chi"],
		kun_yomis: &["ike"]
	},
	Kanji {
		id: 158,
		kanji: '海',
		strokes: 9,
		meanings: &["sea"],
		on_yomis: &["kai"],
		kun_yomis: &["umi"]
	},
	Kanji {
		id: 159,
		kanji: '岩',
		strokes: 8,
		meanings: &["rock"],
		on_yomis: &["gan"],
		kun_yomis: &["iwa"]
	},
	Kanji {
		id: 160,
		kanji: '星',
		strokes: 9,
		meanings: &["star"],
		on_yomis: &["sei"],
		kun_yomis: &["hoshi"]
	},
	Kanji {
		id: 161,
		kanji: '室',
		strokes: 9,
		meanings: &["room"],
		on_yomis: &["shitsu"],
		kun_yomis: &["muro"]
	},
	Kanji {
		id: 162,
		kanji: '戸',
		strokes: 4,
		meanings: &["door"],
		on_yomis: &["ko"],
		kun_yomis: &["to","be"]
	},
	Kanji {
		id: 163,
		kanji: '家',
		strokes: 10,
		meanings: &["house"],
		on_yomis: &["ka","ke"],
		kun_yomis: &["ie"]
	},
	Kanji {
		id: 164,
		kanji: '寺',
		strokes: 6,
		meanings: &["Buddhist temple"],
		on_yomis: &["ji"],
		kun_yomis: &["tera"]
	},
	Kanji {
		id: 165,
		kanji: '通',
		strokes: 10,
		meanings: &["pass through","commute"],
		on_yomis: &["tsū"],
		kun_yomis: &["tō-ru","kayo-u"]
	},
	Kanji {
		id: 166,
		kanji: '門',
		strokes: 8,
		meanings: &["gates"],
		on_yomis: &["mon"],
		kun_yomis: &["kado"]
	},
	Kanji {
		id: 167,
		kanji: '道',
		strokes: 12,
		meanings: &["road"],
		on_yomis: &["dō"],
		kun_yomis: &["michi"]
	},
	Kanji {
		id: 168,
		kanji: '話',
		strokes: 13,
		meanings: &["talk"],
		on_yomis: &["wa"],
		kun_yomis: &["hanashi","hana-su"]
	},
	Kanji {
		id: 169,
		kanji: '言',
		strokes: 7,
		meanings: &["say"],
		on_yomis: &["gen","gon"],
		kun_yomis: &["i-u","koto"]
	},
	Kanji {
		id: 170,
		kanji: '答',
		strokes: 12,
		meanings: &["answer"],
		on_yomis: &["tō"],
		kun_yomis: &["kota-eru"]
	},
	Kanji {
		id: 171,
		kanji: '声',
		strokes: 7,
		meanings: &["voice"],
		on_yomis: &["sei"],
		kun_yomis: &["koe"]
	},
	Kanji {
		id: 172,
		kanji: '聞',
		strokes: 14,
		meanings: &["hear","listen","ask"],
		on_yomis: &["bun","mon"],
		kun_yomis: &["ki-ku"]
	},
	Kanji {
		id: 173,
		kanji: '語',
		strokes: 14,
		meanings: &["language"],
		on_yomis: &["go"],
		kun_yomis: &["kata-ru"]
	},
	Kanji {
		id: 174,
		kanji: '読',
		strokes: 14,
		meanings: &["read"],
		on_yomis: &["doku"],
		kun_yomis: &["yo-mu"]
	},
	Kanji {
		id: 175,
		kanji: '書',
		strokes: 10,
		meanings: &["write"],
		on_yomis: &["sho"],
		kun_yomis: &["ka-ku"]
	},
	Kanji {
		id: 176,
		kanji: '記',
		strokes: 10,
		meanings: &["record"],
		on_yomis: &["ki"],
		kun_yomis: &["shiru-su"]
	},
	Kanji {
		id: 177,
		kanji: '紙',
		strokes: 10,
		meanings: &["paper"],
		on_yomis: &["shi"],
		kun_yomis: &["kami"]
	},
	Kanji {
		id: 178,
		kanji: '画',
		strokes: 8,
		meanings: &["brush stroke"],
		on_yomis: &["ga","kaku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 179,
		kanji: '絵',
		strokes: 12,
		meanings: &["picture"],
		on_yomis: &["kai","e"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 180,
		kanji: '図',
		strokes: 7,
		meanings: &["drawing"],
		on_yomis: &["zu"],
		kun_yomis: &["haka-ru"]
	},
	Kanji {
		id: 181,
		kanji: '工',
		strokes: 3,
		meanings: &["craft"],
		on_yomis: &["kō","ku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 182,
		kanji: '教',
		strokes: 11,
		meanings: &["teach"],
		on_yomis: &["kyō"],
		kun_yomis: &["oshi-eru"]
	},
	Kanji {
		id: 183,
		kanji: '晴',
		strokes: 12,
		meanings: &["clear"],
		on_yomis: &["sei"],
		kun_yomis: &["hare"]
	},
	Kanji {
		id: 184,
		kanji: '思',
		strokes: 9,
		meanings: &["think"],
		on_yomis: &["shi"],
		kun_yomis: &["omo-u"]
	},
	Kanji {
		id: 185,
		kanji: '考',
		strokes: 6,
		meanings: &["consider"],
		on_yomis: &["kō"],
		kun_yomis: &["kanga-eru"]
	},
	Kanji {
		id: 186,
		kanji: '知',
		strokes: 8,
		meanings: &["know"],
		on_yomis: &["chi"],
		kun_yomis: &["shi-ru"]
	},
	Kanji {
		id: 187,
		kanji: '才',
		strokes: 3,
		meanings: &["age","ability"],
		on_yomis: &["sai","zai"],
		kun_yomis: &["wazukani","zae"]
	},
	Kanji {
		id: 188,
		kanji: '理',
		strokes: 11,
		meanings: &["reason"],
		on_yomis: &["ri"],
		kun_yomis: &["kotowari"]
	},
	Kanji {
		id: 189,
		kanji: '算',
		strokes: 14,
		meanings: &["calculate"],
		on_yomis: &["san"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 190,
		kanji: '作',
		strokes: 7,
		meanings: &["make"],
		on_yomis: &["saku"],
		kun_yomis: &["tsuku-ru"]
	},
	Kanji {
		id: 191,
		kanji: '元',
		strokes: 4,
		meanings: &["origin"],
		on_yomis: &["gen","gan"],
		kun_yomis: &["moto"]
	},
	Kanji {
		id: 192,
		kanji: '食',
		strokes: 9,
		meanings: &["eat"],
		on_yomis: &["shoku"],
		kun_yomis: &["ta-beru","ku-u"]
	},
	Kanji {
		id: 193,
		kanji: '肉',
		strokes: 6,
		meanings: &["meat"],
		on_yomis: &["niku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 194,
		kanji: '馬',
		strokes: 10,
		meanings: &["horse"],
		on_yomis: &["ba"],
		kun_yomis: &["uma","ma"]
	},
	Kanji {
		id: 195,
		kanji: '牛',
		strokes: 4,
		meanings: &["cow"],
		on_yomis: &["gyū"],
		kun_yomis: &["ushi"]
	},
	Kanji {
		id: 196,
		kanji: '魚',
		strokes: 11,
		meanings: &["fish"],
		on_yomis: &["gyo"],
		kun_yomis: &["sakana"]
	},
	Kanji {
		id: 197,
		kanji: '鳥',
		strokes: 11,
		meanings: &["bird"],
		on_yomis: &["chō"],
		kun_yomis: &["tori"]
	},
	Kanji {
		id: 198,
		kanji: '羽',
		strokes: 6,
		meanings: &["feather"],
		on_yomis: &["u"],
		kun_yomis: &["ha","hane"]
	},
	Kanji {
		id: 199,
		kanji: '鳴',
		strokes: 14,
		meanings: &["chirp"],
		on_yomis: &["mei"],
		kun_yomis: &["na-ku"]
	},
	Kanji {
		id: 200,
		kanji: '麦',
		strokes: 7,
		meanings: &["wheat"],
		on_yomis: &["baku"],
		kun_yomis: &["mugi"]
	},
	Kanji {
		id: 201,
		kanji: '米',
		strokes: 6,
		meanings: &["rice"],
		on_yomis: &["bei","mai"],
		kun_yomis: &["kome"]
	},
	Kanji {
		id: 202,
		kanji: '茶',
		strokes: 9,
		meanings: &["tea"],
		on_yomis: &["cha","sa"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 203,
		kanji: '色',
		strokes: 6,
		meanings: &["colour"],
		on_yomis: &["shoku"],
		kun_yomis: &["iro"]
	},
	Kanji {
		id: 204,
		kanji: '黄',
		strokes: 11,
		meanings: &["yellow"],
		on_yomis: &["ō"],
		kun_yomis: &["ki"]
	},
	Kanji {
		id: 205,
		kanji: '黒',
		strokes: 11,
		meanings: &["black"],
		on_yomis: &["koku"],
		kun_yomis: &["kuro"]
	},
	Kanji {
		id: 206,
		kanji: '来',
		strokes: 7,
		meanings: &["come"],
		on_yomis: &["rai"],
		kun_yomis: &["ku-ru"]
	},
	Kanji {
		id: 207,
		kanji: '行',
		strokes: 6,
		meanings: &["go"],
		on_yomis: &["kō","gyō"],
		kun_yomis: &["i-ku","yu-ku","okona-u"]
	},
	Kanji {
		id: 208,
		kanji: '帰',
		strokes: 10,
		meanings: &["return"],
		on_yomis: &["ki"],
		kun_yomis: &["kae-ru"]
	},
	Kanji {
		id: 209,
		kanji: '歩',
		strokes: 8,
		meanings: &["walk"],
		on_yomis: &["ho","fu","bu"],
		kun_yomis: &["aru-ku","ayu-mu"]
	},
	Kanji {
		id: 210,
		kanji: '走',
		strokes: 7,
		meanings: &["run"],
		on_yomis: &["sou"],
		kun_yomis: &["hashi-ru"]
	},
	Kanji {
		id: 211,
		kanji: '止',
		strokes: 4,
		meanings: &["stop"],
		on_yomis: &["shi"],
		kun_yomis: &["to-maru"]
	},
	Kanji {
		id: 212,
		kanji: '活',
		strokes: 9,
		meanings: &["active"],
		on_yomis: &["katsu"],
		kun_yomis: &["i-kiru"]
	},
	Kanji {
		id: 213,
		kanji: '店',
		strokes: 8,
		meanings: &["store"],
		on_yomis: &["ten"],
		kun_yomis: &["mise"]
	},
	Kanji {
		id: 214,
		kanji: '買',
		strokes: 12,
		meanings: &["buy"],
		on_yomis: &["bai"],
		kun_yomis: &["ka-u"]
	},
	Kanji {
		id: 215,
		kanji: '売',
		strokes: 7,
		meanings: &["sell"],
		on_yomis: &["bai"],
		kun_yomis: &["u-ru"]
	},
	Kanji {
		id: 216,
		kanji: '午',
		strokes: 4,
		meanings: &["noon"],
		on_yomis: &["go"],
		kun_yomis: &["uma"]
	},
	Kanji {
		id: 217,
		kanji: '汽',
		strokes: 7,
		meanings: &["steam"],
		on_yomis: &["ki"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 218,
		kanji: '弓',
		strokes: 3,
		meanings: &["bow"],
		on_yomis: &["kyū"],
		kun_yomis: &["yumi"]
	},
	Kanji {
		id: 219,
		kanji: '回',
		strokes: 6,
		meanings: &["number of times","revolve"],
		on_yomis: &["kai"],
		kun_yomis: &["mawa-ru"]
	},
	Kanji {
		id: 220,
		kanji: '会',
		strokes: 6,
		meanings: &["meet"],
		on_yomis: &["kai","e"],
		kun_yomis: &["a-u"]
	},
	Kanji {
		id: 221,
		kanji: '組',
		strokes: 11,
		meanings: &["team"],
		on_yomis: &["so"],
		kun_yomis: &["kumi"]
	},
	Kanji {
		id: 222,
		kanji: '船',
		strokes: 11,
		meanings: &["ship"],
		on_yomis: &["sen"],
		kun_yomis: &["fune"]
	},
	Kanji {
		id: 223,
		kanji: '明',
		strokes: 8,
		meanings: &["bright"],
		on_yomis: &["mei"],
		kun_yomis: &["aka-rui"]
	},
	Kanji {
		id: 224,
		kanji: '社',
		strokes: 7,
		meanings: &["company"],
		on_yomis: &["sha"],
		kun_yomis: &["yashiro"]
	},
	Kanji {
		id: 225,
		kanji: '切',
		strokes: 4,
		meanings: &["cut"],
		on_yomis: &["setsu"],
		kun_yomis: &["ki-ru"]
	},
	Kanji {
		id: 226,
		kanji: '電',
		strokes: 13,
		meanings: &["electricity"],
		on_yomis: &["den"],
		kun_yomis: &["inazuma"]
	},
	Kanji {
		id: 227,
		kanji: '毎',
		strokes: 6,
		meanings: &["every"],
		on_yomis: &["mai"],
		kun_yomis: &["goto"]
	},
	Kanji {
		id: 228,
		kanji: '合',
		strokes: 6,
		meanings: &["fit"],
		on_yomis: &["gō"],
		kun_yomis: &["a-u"]
	},
	Kanji {
		id: 229,
		kanji: '当',
		strokes: 6,
		meanings: &["this","hit"],
		on_yomis: &["tō"],
		kun_yomis: &["a-taru"]
	},
	Kanji {
		id: 230,
		kanji: '台',
		strokes: 5,
		meanings: &["pedestal"],
		on_yomis: &["dai","tai"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 231,
		kanji: '楽',
		strokes: 13,
		meanings: &["music","pleasure"],
		on_yomis: &["gaku","raku"],
		kun_yomis: &["tano-shii"]
	},
	Kanji {
		id: 232,
		kanji: '公',
		strokes: 4,
		meanings: &["public"],
		on_yomis: &["kō"],
		kun_yomis: &["ōyake"]
	},
	Kanji {
		id: 233,
		kanji: '引',
		strokes: 4,
		meanings: &["pull"],
		on_yomis: &["in"],
		kun_yomis: &["hi-ku"]
	},
	Kanji {
		id: 234,
		kanji: '科',
		strokes: 9,
		meanings: &["section","grade"],
		on_yomis: &["ka"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 235,
		kanji: '歌',
		strokes: 14,
		meanings: &["song"],
		on_yomis: &["ka"],
		kun_yomis: &["uta"]
	},
	Kanji {
		id: 236,
		kanji: '刀',
		strokes: 2,
		meanings: &["sword"],
		on_yomis: &["tō"],
		kun_yomis: &["katana"]
	},
	Kanji {
		id: 237,
		kanji: '番',
		strokes: 12,
		meanings: &["number"],
		on_yomis: &["ban"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 238,
		kanji: '用',
		strokes: 5,
		meanings: &["use"],
		on_yomis: &["yō"],
		kun_yomis: &["mochi-iru"]
	},
	Kanji {
		id: 239,
		kanji: '何',
		strokes: 7,
		meanings: &["what"],
		on_yomis: &["ka"],
		kun_yomis: &["nani","nan"]
	},
	Kanji {
		id: 240,
		kanji: '丁',
		strokes: 2,
		meanings: &["street","district"],
		on_yomis: &["chō"],
		kun_yomis: &["hinoto"]
	},
	Kanji {
		id: 241,
		kanji: '世',
		strokes: 5,
		meanings: &["generation"],
		on_yomis: &["sei","se"],
		kun_yomis: &["yo"]
	},
	Kanji {
		id: 242,
		kanji: '両',
		strokes: 6,
		meanings: &["both"],
		on_yomis: &["ryō"],
		kun_yomis: &["teru","futatsu"]
	},
	Kanji {
		id: 243,
		kanji: '主',
		strokes: 5,
		meanings: &["master","main"],
		on_yomis: &["shu"],
		kun_yomis: &["nushi","omo"]
	},
	Kanji {
		id: 244,
		kanji: '乗',
		strokes: 9,
		meanings: &["ride"],
		on_yomis: &["jō"],
		kun_yomis: &["no-ru"]
	},
	Kanji {
		id: 245,
		kanji: '予',
		strokes: 4,
		meanings: &["beforehand"],
		on_yomis: &["yo"],
		kun_yomis: &["arakaji-me"]
	},
	Kanji {
		id: 246,
		kanji: '事',
		strokes: 8,
		meanings: &["intangible thing"],
		on_yomis: &["ji"],
		kun_yomis: &["koto"]
	},
	Kanji {
		id: 247,
		kanji: '仕',
		strokes: 5,
		meanings: &["serve"],
		on_yomis: &["shi"],
		kun_yomis: &["tsuka-eru"]
	},
	Kanji {
		id: 248,
		kanji: '他',
		strokes: 5,
		meanings: &["other"],
		on_yomis: &["ta"],
		kun_yomis: &["hoka"]
	},
	Kanji {
		id: 249,
		kanji: '代',
		strokes: 5,
		meanings: &["era","substitute"],
		on_yomis: &["dai","tai"],
		kun_yomis: &["ka-waru","yo"]
	},
	Kanji {
		id: 250,
		kanji: '住',
		strokes: 7,
		meanings: &["dwell"],
		on_yomis: &["jū"],
		kun_yomis: &["su-mu"]
	},
	Kanji {
		id: 251,
		kanji: '使',
		strokes: 8,
		meanings: &["use"],
		on_yomis: &["shi"],
		kun_yomis: &["tsuka-u"]
	},
	Kanji {
		id: 252,
		kanji: '係',
		strokes: 9,
		meanings: &["person in charge"],
		on_yomis: &["kei"],
		kun_yomis: &["kakari","kaka-ru"]
	},
	Kanji {
		id: 253,
		kanji: '倍',
		strokes: 10,
		meanings: &["double"],
		on_yomis: &["bai"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 254,
		kanji: '全',
		strokes: 6,
		meanings: &["whole"],
		on_yomis: &["zen"],
		kun_yomis: &["matta-ku"]
	},
	Kanji {
		id: 255,
		kanji: '具',
		strokes: 8,
		meanings: &["tool"],
		on_yomis: &["gu"],
		kun_yomis: &["sona-eru","tsubusa-ni"]
	},
	Kanji {
		id: 256,
		kanji: '写',
		strokes: 5,
		meanings: &["copy"],
		on_yomis: &["sha"],
		kun_yomis: &["utsu-su"]
	},
	Kanji {
		id: 257,
		kanji: '列',
		strokes: 6,
		meanings: &["row"],
		on_yomis: &["retsu"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 258,
		kanji: '助',
		strokes: 7,
		meanings: &["help"],
		on_yomis: &["jo"],
		kun_yomis: &["tasu-keru"]
	},
	Kanji {
		id: 259,
		kanji: '勉',
		strokes: 10,
		meanings: &["diligence"],
		on_yomis: &["ben"],
		kun_yomis: &["tsuto-meru"]
	},
	Kanji {
		id: 260,
		kanji: '動',
		strokes: 11,
		meanings: &["move"],
		on_yomis: &["dō"],
		kun_yomis: &["ugo-ku"]
	},
	Kanji {
		id: 261,
		kanji: '勝',
		strokes: 12,
		meanings: &["win"],
		on_yomis: &["shō"],
		kun_yomis: &["ka-tsu"]
	},
	Kanji {
		id: 262,
		kanji: '化',
		strokes: 4,
		meanings: &["change"],
		on_yomis: &["ka"],
		kun_yomis: &["ba-keru"]
	},
	Kanji {
		id: 263,
		kanji: '区',
		strokes: 4,
		meanings: &["district"],
		on_yomis: &["ku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 264,
		kanji: '医',
		strokes: 7,
		meanings: &["doctor"],
		on_yomis: &["i"],
		kun_yomis: &["iya-su","i-suru"]
	},
	Kanji {
		id: 265,
		kanji: '去',
		strokes: 5,
		meanings: &["leave"],
		on_yomis: &["kyo","ko"],
		kun_yomis: &["sa-ru"]
	},
	Kanji {
		id: 266,
		kanji: '反',
		strokes: 4,
		meanings: &["anti-"],
		on_yomis: &["han"],
		kun_yomis: &["so-ru"]
	},
	Kanji {
		id: 267,
		kanji: '取',
		strokes: 8,
		meanings: &["take"],
		on_yomis: &["shu"],
		kun_yomis: &["to-ru"]
	},
	Kanji {
		id: 268,
		kanji: '受',
		strokes: 8,
		meanings: &["receive"],
		on_yomis: &["ju"],
		kun_yomis: &["u-keru"]
	},
	Kanji {
		id: 269,
		kanji: '号',
		strokes: 5,
		meanings: &["number"],
		on_yomis: &["gō"],
		kun_yomis: &["yobina","sake-bu"]
	},
	Kanji {
		id: 270,
		kanji: '向',
		strokes: 6,
		meanings: &["face"],
		on_yomis: &["kō"],
		kun_yomis: &["mu-kau"]
	},
	Kanji {
		id: 271,
		kanji: '君',
		strokes: 7,
		meanings: &["you","monarch"],
		on_yomis: &["kun"],
		kun_yomis: &["kimi"]
	},
	Kanji {
		id: 272,
		kanji: '味',
		strokes: 8,
		meanings: &["flavor"],
		on_yomis: &["mi"],
		kun_yomis: &["aji","aji-wau"]
	},
	Kanji {
		id: 273,
		kanji: '命',
		strokes: 8,
		meanings: &["fate","life"],
		on_yomis: &["mei"],
		kun_yomis: &["inochi"]
	},
	Kanji {
		id: 274,
		kanji: '和',
		strokes: 8,
		meanings: &["harmony","Japanese"],
		on_yomis: &["wa"],
		kun_yomis: &["yawa-ragu","nago-yaka"]
	},
	Kanji {
		id: 275,
		kanji: '品',
		strokes: 9,
		meanings: &["article"],
		on_yomis: &["hin"],
		kun_yomis: &["shina"]
	},
	Kanji {
		id: 276,
		kanji: '員',
		strokes: 10,
		meanings: &["employee"],
		on_yomis: &["in"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 277,
		kanji: '商',
		strokes: 11,
		meanings: &["commerce"],
		on_yomis: &["shō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 278,
		kanji: '問',
		strokes: 11,
		meanings: &["question"],
		on_yomis: &["mon"],
		kun_yomis: &["to-u","ton"]
	},
	Kanji {
		id: 279,
		kanji: '坂',
		strokes: 7,
		meanings: &["slope"],
		on_yomis: &["han"],
		kun_yomis: &["saka"]
	},
	Kanji {
		id: 280,
		kanji: '央',
		strokes: 5,
		meanings: &["center"],
		on_yomis: &["ō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 281,
		kanji: '始',
		strokes: 8,
		meanings: &["begin"],
		on_yomis: &["shi"],
		kun_yomis: &["haji-meru"]
	},
	Kanji {
		id: 282,
		kanji: '委',
		strokes: 8,
		meanings: &["committee"],
		on_yomis: &["i"],
		kun_yomis: &["yuda-neru"]
	},
	Kanji {
		id: 283,
		kanji: '守',
		strokes: 6,
		meanings: &["protect"],
		on_yomis: &["shu"],
		kun_yomis: &["mamo-ru"]
	},
	Kanji {
		id: 284,
		kanji: '安',
		strokes: 6,
		meanings: &["cheap","calm"],
		on_yomis: &["an"],
		kun_yomis: &["yasu-i"]
	},
	Kanji {
		id: 285,
		kanji: '定',
		strokes: 8,
		meanings: &["determine"],
		on_yomis: &["tei","jō"],
		kun_yomis: &["sada-meru"]
	},
	Kanji {
		id: 286,
		kanji: '実',
		strokes: 8,
		meanings: &["fruit","realization"],
		on_yomis: &["jitsu"],
		kun_yomis: &["mi","mino-ru"]
	},
	Kanji {
		id: 287,
		kanji: '客',
		strokes: 9,
		meanings: &["guest"],
		on_yomis: &["kyaku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 288,
		kanji: '宮',
		strokes: 10,
		meanings: &["Shinto shrine","prince"],
		on_yomis: &["kyū","gū"],
		kun_yomis: &["miya"]
	},
	Kanji {
		id: 289,
		kanji: '宿',
		strokes: 11,
		meanings: &["inn"],
		on_yomis: &["shuku"],
		kun_yomis: &["yado","yado-ru"]
	},
	Kanji {
		id: 290,
		kanji: '寒',
		strokes: 12,
		meanings: &["cold (weather)"],
		on_yomis: &["kan"],
		kun_yomis: &["samu-i"]
	},
	Kanji {
		id: 291,
		kanji: '対',
		strokes: 7,
		meanings: &["opposite","against"],
		on_yomis: &["tai","tsui"],
		kun_yomis: &["aite","soro-i"]
	},
	Kanji {
		id: 292,
		kanji: '局',
		strokes: 7,
		meanings: &["office"],
		on_yomis: &["kyoku"],
		kun_yomis: &["tsubone"]
	},
	Kanji {
		id: 293,
		kanji: '屋',
		strokes: 9,
		meanings: &["roof"],
		on_yomis: &["oku"],
		kun_yomis: &["ya"]
	},
	Kanji {
		id: 294,
		kanji: '岸',
		strokes: 8,
		meanings: &["shore"],
		on_yomis: &["gan"],
		kun_yomis: &["kishi"]
	},
	Kanji {
		id: 295,
		kanji: '島',
		strokes: 10,
		meanings: &["island"],
		on_yomis: &["tō"],
		kun_yomis: &["shima"]
	},
	Kanji {
		id: 296,
		kanji: '州',
		strokes: 6,
		meanings: &["state","province"],
		on_yomis: &["shū"],
		kun_yomis: &["su"]
	},
	Kanji {
		id: 297,
		kanji: '帳',
		strokes: 11,
		meanings: &["notebook"],
		on_yomis: &["chō"],
		kun_yomis: &["tobari"]
	},
	Kanji {
		id: 298,
		kanji: '平',
		strokes: 5,
		meanings: &["flat"],
		on_yomis: &["hei","byō"],
		kun_yomis: &["tai-ra","hira"]
	},
	Kanji {
		id: 299,
		kanji: '幸',
		strokes: 8,
		meanings: &["happiness"],
		on_yomis: &["kō"],
		kun_yomis: &["saiwa-i","shiawa-se"]
	},
	Kanji {
		id: 300,
		kanji: '度',
		strokes: 9,
		meanings: &["degree"],
		on_yomis: &["do"],
		kun_yomis: &["tabi"]
	},
	Kanji {
		id: 301,
		kanji: '庫',
		strokes: 10,
		meanings: &["warehouse"],
		on_yomis: &["ko","ku"],
		kun_yomis: &["kura"]
	},
	Kanji {
		id: 302,
		kanji: '庭',
		strokes: 10,
		meanings: &["yard"],
		on_yomis: &["tei"],
		kun_yomis: &["niwa"]
	},
	Kanji {
		id: 303,
		kanji: '式',
		strokes: 6,
		meanings: &["style","ceremony","numerical formula"],
		on_yomis: &["shiki"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 304,
		kanji: '役',
		strokes: 7,
		meanings: &["role"],
		on_yomis: &["yaku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 305,
		kanji: '待',
		strokes: 9,
		meanings: &["wait"],
		on_yomis: &["tai"],
		kun_yomis: &["ma-tsu"]
	},
	Kanji {
		id: 306,
		kanji: '急',
		strokes: 9,
		meanings: &["hurry"],
		on_yomis: &["kyū"],
		kun_yomis: &["iso-gu"]
	},
	Kanji {
		id: 307,
		kanji: '息',
		strokes: 10,
		meanings: &["breath"],
		on_yomis: &["soku"],
		kun_yomis: &["iki"]
	},
	Kanji {
		id: 308,
		kanji: '悪',
		strokes: 11,
		meanings: &["bad"],
		on_yomis: &["aku"],
		kun_yomis: &["waru-i"]
	},
	Kanji {
		id: 309,
		kanji: '悲',
		strokes: 12,
		meanings: &["sad"],
		on_yomis: &["hi"],
		kun_yomis: &["kana-shii"]
	},
	Kanji {
		id: 310,
		kanji: '想',
		strokes: 13,
		meanings: &["thought"],
		on_yomis: &["sō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 311,
		kanji: '意',
		strokes: 13,
		meanings: &["idea"],
		on_yomis: &["i"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 312,
		kanji: '感',
		strokes: 13,
		meanings: &["feel"],
		on_yomis: &["kan"],
		kun_yomis: &["kan-jiru"]
	},
	Kanji {
		id: 313,
		kanji: '所',
		strokes: 8,
		meanings: &["place"],
		on_yomis: &["sho"],
		kun_yomis: &["tokoro"]
	},
	Kanji {
		id: 314,
		kanji: '打',
		strokes: 5,
		meanings: &["hit"],
		on_yomis: &["da"],
		kun_yomis: &["u-tsu"]
	},
	Kanji {
		id: 315,
		kanji: '投',
		strokes: 7,
		meanings: &["throw"],
		on_yomis: &["tō"],
		kun_yomis: &["na-geru"]
	},
	Kanji {
		id: 316,
		kanji: '拾',
		strokes: 9,
		meanings: &["pick up"],
		on_yomis: &["shū"],
		kun_yomis: &["hiro-u"]
	},
	Kanji {
		id: 317,
		kanji: '持',
		strokes: 9,
		meanings: &["hold"],
		on_yomis: &["ji"],
		kun_yomis: &["mo-tsu"]
	},
	Kanji {
		id: 318,
		kanji: '指',
		strokes: 9,
		meanings: &["finger","point"],
		on_yomis: &["shi"],
		kun_yomis: &["yubi","sa-su"]
	},
	Kanji {
		id: 319,
		kanji: '放',
		strokes: 8,
		meanings: &["release"],
		on_yomis: &["hō"],
		kun_yomis: &["hana-su"]
	},
	Kanji {
		id: 320,
		kanji: '整',
		strokes: 16,
		meanings: &["organize"],
		on_yomis: &["sei"],
		kun_yomis: &["totono-eru"]
	},
	Kanji {
		id: 321,
		kanji: '旅',
		strokes: 10,
		meanings: &["trip"],
		on_yomis: &["ryo"],
		kun_yomis: &["tabi"]
	},
	Kanji {
		id: 322,
		kanji: '族',
		strokes: 11,
		meanings: &["tribe"],
		on_yomis: &["zoku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 323,
		kanji: '昔',
		strokes: 8,
		meanings: &["long ago"],
		on_yomis: &["seki","shaku"],
		kun_yomis: &["mukashi"]
	},
	Kanji {
		id: 324,
		kanji: '昭',
		strokes: 9,
		meanings: &["shine"],
		on_yomis: &["shō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 325,
		kanji: '暑',
		strokes: 12,
		meanings: &["hot"],
		on_yomis: &["sho"],
		kun_yomis: &["atsu-i"]
	},
	Kanji {
		id: 326,
		kanji: '暗',
		strokes: 13,
		meanings: &["dark"],
		on_yomis: &["an"],
		kun_yomis: &["kura-i"]
	},
	Kanji {
		id: 327,
		kanji: '曲',
		strokes: 6,
		meanings: &["melody","curve"],
		on_yomis: &["kyoku"],
		kun_yomis: &["ma-garu"]
	},
	Kanji {
		id: 328,
		kanji: '有',
		strokes: 6,
		meanings: &["possess"],
		on_yomis: &["yū"],
		kun_yomis: &["a-ru"]
	},
	Kanji {
		id: 329,
		kanji: '服',
		strokes: 8,
		meanings: &["clothes"],
		on_yomis: &["fuku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 330,
		kanji: '期',
		strokes: 12,
		meanings: &["period of time"],
		on_yomis: &["ki"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 331,
		kanji: '板',
		strokes: 8,
		meanings: &["board"],
		on_yomis: &["han","ban"],
		kun_yomis: &["ita"]
	},
	Kanji {
		id: 332,
		kanji: '柱',
		strokes: 9,
		meanings: &["pillar"],
		on_yomis: &["chū"],
		kun_yomis: &["hashira"]
	},
	Kanji {
		id: 333,
		kanji: '根',
		strokes: 10,
		meanings: &["root"],
		on_yomis: &["kon"],
		kun_yomis: &["ne"]
	},
	Kanji {
		id: 334,
		kanji: '植',
		strokes: 12,
		meanings: &["plant"],
		on_yomis: &["shoku"],
		kun_yomis: &["u-eru"]
	},
	Kanji {
		id: 335,
		kanji: '業',
		strokes: 13,
		meanings: &["business"],
		on_yomis: &["gyō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 336,
		kanji: '様',
		strokes: 14,
		meanings: &["appearance"],
		on_yomis: &["yō"],
		kun_yomis: &["sama"]
	},
	Kanji {
		id: 337,
		kanji: '横',
		strokes: 15,
		meanings: &["horizontal"],
		on_yomis: &["ō"],
		kun_yomis: &["yoko"]
	},
	Kanji {
		id: 338,
		kanji: '橋',
		strokes: 16,
		meanings: &["bridge"],
		on_yomis: &["kyō"],
		kun_yomis: &["hashi"]
	},
	Kanji {
		id: 339,
		kanji: '次',
		strokes: 6,
		meanings: &["next"],
		on_yomis: &["ji"],
		kun_yomis: &["tsugi","tsu-gu"]
	},
	Kanji {
		id: 340,
		kanji: '歯',
		strokes: 12,
		meanings: &["tooth"],
		on_yomis: &["shi"],
		kun_yomis: &["ha"]
	},
	Kanji {
		id: 341,
		kanji: '死',
		strokes: 6,
		meanings: &["death"],
		on_yomis: &["shi"],
		kun_yomis: &["shi-nu"]
	},
	Kanji {
		id: 342,
		kanji: '氷',
		strokes: 5,
		meanings: &["ice"],
		on_yomis: &["hyō"],
		kun_yomis: &["kōri"]
	},
	Kanji {
		id: 343,
		kanji: '決',
		strokes: 7,
		meanings: &["decide"],
		on_yomis: &["ketsu"],
		kun_yomis: &["ki-meru"]
	},
	Kanji {
		id: 344,
		kanji: '油',
		strokes: 8,
		meanings: &["oil"],
		on_yomis: &["yu"],
		kun_yomis: &["abura"]
	},
	Kanji {
		id: 345,
		kanji: '波',
		strokes: 8,
		meanings: &["wave"],
		on_yomis: &["ha"],
		kun_yomis: &["nami"]
	},
	Kanji {
		id: 346,
		kanji: '注',
		strokes: 8,
		meanings: &["pour"],
		on_yomis: &["chū"],
		kun_yomis: &["soso-gu"]
	},
	Kanji {
		id: 347,
		kanji: '泳',
		strokes: 8,
		meanings: &["swim"],
		on_yomis: &["ei"],
		kun_yomis: &["oyo-gu"]
	},
	Kanji {
		id: 348,
		kanji: '洋',
		strokes: 9,
		meanings: &["ocean"],
		on_yomis: &["yō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 349,
		kanji: '流',
		strokes: 10,
		meanings: &["stream"],
		on_yomis: &["ryū"],
		kun_yomis: &["naga-reru"]
	},
	Kanji {
		id: 350,
		kanji: '消',
		strokes: 10,
		meanings: &["extinguish"],
		on_yomis: &["shō"],
		kun_yomis: &["ki-eru","ke-su"]
	},
	Kanji {
		id: 351,
		kanji: '深',
		strokes: 11,
		meanings: &["deep"],
		on_yomis: &["shin"],
		kun_yomis: &["fuka-i"]
	},
	Kanji {
		id: 352,
		kanji: '温',
		strokes: 12,
		meanings: &["warm"],
		on_yomis: &["on"],
		kun_yomis: &["atata-kai"]
	},
	Kanji {
		id: 353,
		kanji: '港',
		strokes: 12,
		meanings: &["harbor"],
		on_yomis: &["kō"],
		kun_yomis: &["minato"]
	},
	Kanji {
		id: 354,
		kanji: '湖',
		strokes: 12,
		meanings: &["lake"],
		on_yomis: &["ko"],
		kun_yomis: &["mizu'umi"]
	},
	Kanji {
		id: 355,
		kanji: '湯',
		strokes: 12,
		meanings: &["hot water"],
		on_yomis: &["tō"],
		kun_yomis: &["yu"]
	},
	Kanji {
		id: 356,
		kanji: '漢',
		strokes: 13,
		meanings: &["Chinese"],
		on_yomis: &["kan"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 357,
		kanji: '炭',
		strokes: 9,
		meanings: &["charcoal"],
		on_yomis: &["tan"],
		kun_yomis: &["sumi"]
	},
	Kanji {
		id: 358,
		kanji: '物',
		strokes: 8,
		meanings: &["(tangible) thing"],
		on_yomis: &["butsu","motsu"],
		kun_yomis: &["mono"]
	},
	Kanji {
		id: 359,
		kanji: '球',
		strokes: 11,
		meanings: &["sphere"],
		on_yomis: &["kyū"],
		kun_yomis: &["tama"]
	},
	Kanji {
		id: 360,
		kanji: '由',
		strokes: 5,
		meanings: &["reason"],
		on_yomis: &["yū","yu"],
		kun_yomis: &["yoshi"]
	},
	Kanji {
		id: 361,
		kanji: '申',
		strokes: 5,
		meanings: &["say"],
		on_yomis: &["shin"],
		kun_yomis: &["mō-su"]
	},
	Kanji {
		id: 362,
		kanji: '界',
		strokes: 9,
		meanings: &["world"],
		on_yomis: &["kai"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 363,
		kanji: '畑',
		strokes: 9,
		meanings: &["farm"],
		on_yomis: &[""],
		kun_yomis: &["hata","hatake"]
	},
	Kanji {
		id: 364,
		kanji: '病',
		strokes: 10,
		meanings: &["sick"],
		on_yomis: &["byō"],
		kun_yomis: &["yamai"]
	},
	Kanji {
		id: 365,
		kanji: '発',
		strokes: 9,
		meanings: &["departure"],
		on_yomis: &["hatsu"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 366,
		kanji: '登',
		strokes: 12,
		meanings: &["climb"],
		on_yomis: &["tō","to"],
		kun_yomis: &["nobo-ru"]
	},
	Kanji {
		id: 367,
		kanji: '皮',
		strokes: 5,
		meanings: &["skin"],
		on_yomis: &["hi"],
		kun_yomis: &["kawa"]
	},
	Kanji {
		id: 368,
		kanji: '皿',
		strokes: 5,
		meanings: &["dish"],
		on_yomis: &["bei"],
		kun_yomis: &["sara"]
	},
	Kanji {
		id: 369,
		kanji: '相',
		strokes: 9,
		meanings: &["mutual"],
		on_yomis: &["sō"],
		kun_yomis: &["ai"]
	},
	Kanji {
		id: 370,
		kanji: '県',
		strokes: 9,
		meanings: &["prefecture"],
		on_yomis: &["ken"],
		kun_yomis: &["ka-keru"]
	},
	Kanji {
		id: 371,
		kanji: '真',
		strokes: 10,
		meanings: &["true"],
		on_yomis: &["shin"],
		kun_yomis: &["ma"]
	},
	Kanji {
		id: 372,
		kanji: '着',
		strokes: 12,
		meanings: &["wear","arrive"],
		on_yomis: &["chaku"],
		kun_yomis: &["ki-ru","tsu-ku"]
	},
	Kanji {
		id: 373,
		kanji: '短',
		strokes: 12,
		meanings: &["short"],
		on_yomis: &["tan"],
		kun_yomis: &["mijika-i"]
	},
	Kanji {
		id: 374,
		kanji: '研',
		strokes: 9,
		meanings: &["sharpen"],
		on_yomis: &["ken"],
		kun_yomis: &["to-gu"]
	},
	Kanji {
		id: 375,
		kanji: '礼',
		strokes: 5,
		meanings: &["manners"],
		on_yomis: &["rei"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 376,
		kanji: '神',
		strokes: 9,
		meanings: &["deity"],
		on_yomis: &["shin","jin"],
		kun_yomis: &["kami"]
	},
	Kanji {
		id: 377,
		kanji: '祭',
		strokes: 11,
		meanings: &["festival"],
		on_yomis: &["sai"],
		kun_yomis: &["matsu-ri"]
	},
	Kanji {
		id: 378,
		kanji: '福',
		strokes: 13,
		meanings: &["luck"],
		on_yomis: &["fuku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 379,
		kanji: '秒',
		strokes: 9,
		meanings: &["second"],
		on_yomis: &["byō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 380,
		kanji: '究',
		strokes: 7,
		meanings: &["research"],
		on_yomis: &["kyū"],
		kun_yomis: &["kiwa-meru"]
	},
	Kanji {
		id: 381,
		kanji: '章',
		strokes: 11,
		meanings: &["chapter"],
		on_yomis: &["shō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 382,
		kanji: '童',
		strokes: 12,
		meanings: &["juvenile"],
		on_yomis: &["dō"],
		kun_yomis: &["warabe"]
	},
	Kanji {
		id: 383,
		kanji: '笛',
		strokes: 11,
		meanings: &["flute"],
		on_yomis: &["teki"],
		kun_yomis: &["fue"]
	},
	Kanji {
		id: 384,
		kanji: '第',
		strokes: 11,
		meanings: &["ordinal"],
		on_yomis: &["dai"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 385,
		kanji: '筆',
		strokes: 12,
		meanings: &["writing brush"],
		on_yomis: &["hitsu"],
		kun_yomis: &["fude"]
	},
	Kanji {
		id: 386,
		kanji: '等',
		strokes: 12,
		meanings: &["class"],
		on_yomis: &["tō"],
		kun_yomis: &["hito-shii"]
	},
	Kanji {
		id: 387,
		kanji: '箱',
		strokes: 15,
		meanings: &["box"],
		on_yomis: &["sō"],
		kun_yomis: &["hako"]
	},
	Kanji {
		id: 388,
		kanji: '級',
		strokes: 9,
		meanings: &["rank"],
		on_yomis: &["kyū"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 389,
		kanji: '終',
		strokes: 11,
		meanings: &["end"],
		on_yomis: &["shū"],
		kun_yomis: &["o-waru","o-eru"]
	},
	Kanji {
		id: 390,
		kanji: '緑',
		strokes: 14,
		meanings: &["green"],
		on_yomis: &["ryoku"],
		kun_yomis: &["midori"]
	},
	Kanji {
		id: 391,
		kanji: '練',
		strokes: 14,
		meanings: &["practice"],
		on_yomis: &["ren"],
		kun_yomis: &["ne-ru"]
	},
	Kanji {
		id: 392,
		kanji: '羊',
		strokes: 6,
		meanings: &["sheep"],
		on_yomis: &["yō"],
		kun_yomis: &["hitsuji"]
	},
	Kanji {
		id: 393,
		kanji: '美',
		strokes: 9,
		meanings: &["beauty"],
		on_yomis: &["bi"],
		kun_yomis: &["utsuku-shii"]
	},
	Kanji {
		id: 394,
		kanji: '習',
		strokes: 11,
		meanings: &["learn"],
		on_yomis: &["shū"],
		kun_yomis: &["nara-u"]
	},
	Kanji {
		id: 395,
		kanji: '者',
		strokes: 8,
		meanings: &["someone"],
		on_yomis: &["sha"],
		kun_yomis: &["mono"]
	},
	Kanji {
		id: 396,
		kanji: '育',
		strokes: 8,
		meanings: &["raise"],
		on_yomis: &["iku"],
		kun_yomis: &["soda-tsu"]
	},
	Kanji {
		id: 397,
		kanji: '苦',
		strokes: 8,
		meanings: &["suffer","bitter"],
		on_yomis: &["ku"],
		kun_yomis: &["kuru-shii","niga-i"]
	},
	Kanji {
		id: 398,
		kanji: '荷',
		strokes: 10,
		meanings: &["luggage"],
		on_yomis: &["ka"],
		kun_yomis: &["ni"]
	},
	Kanji {
		id: 399,
		kanji: '落',
		strokes: 12,
		meanings: &["fall"],
		on_yomis: &["raku"],
		kun_yomis: &["o-chiru","o-tosu"]
	},
	Kanji {
		id: 400,
		kanji: '葉',
		strokes: 12,
		meanings: &["leaf"],
		on_yomis: &["yō"],
		kun_yomis: &["ha"]
	},
	Kanji {
		id: 401,
		kanji: '薬',
		strokes: 16,
		meanings: &["medicine"],
		on_yomis: &["yaku"],
		kun_yomis: &["kusuri"]
	},
	Kanji {
		id: 402,
		kanji: '血',
		strokes: 6,
		meanings: &["blood"],
		on_yomis: &["ketsu"],
		kun_yomis: &["chi"]
	},
	Kanji {
		id: 403,
		kanji: '表',
		strokes: 8,
		meanings: &["express"],
		on_yomis: &["hyō"],
		kun_yomis: &["omote","arawa-su"]
	},
	Kanji {
		id: 404,
		kanji: '詩',
		strokes: 13,
		meanings: &["poem"],
		on_yomis: &["shi"],
		kun_yomis: &["uta"]
	},
	Kanji {
		id: 405,
		kanji: '調',
		strokes: 15,
		meanings: &["tone","find"],
		on_yomis: &["chō"],
		kun_yomis: &["shira-beru"]
	},
	Kanji {
		id: 406,
		kanji: '談',
		strokes: 15,
		meanings: &["discuss"],
		on_yomis: &["dan"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 407,
		kanji: '豆',
		strokes: 7,
		meanings: &["beans"],
		on_yomis: &["tō","zu"],
		kun_yomis: &["mame"]
	},
	Kanji {
		id: 408,
		kanji: '負',
		strokes: 9,
		meanings: &["lose"],
		on_yomis: &["fu"],
		kun_yomis: &["ma-keru","o-u"]
	},
	Kanji {
		id: 409,
		kanji: '起',
		strokes: 10,
		meanings: &["awaken"],
		on_yomis: &["ki"],
		kun_yomis: &["o-kiru"]
	},
	Kanji {
		id: 410,
		kanji: '路',
		strokes: 13,
		meanings: &["path"],
		on_yomis: &["ro"],
		kun_yomis: &["ji"]
	},
	Kanji {
		id: 411,
		kanji: '身',
		strokes: 7,
		meanings: &["body"],
		on_yomis: &["shin"],
		kun_yomis: &["mi"]
	},
	Kanji {
		id: 412,
		kanji: '転',
		strokes: 11,
		meanings: &["to shift","fall down"],
		on_yomis: &["ten"],
		kun_yomis: &["koro-bu"]
	},
	Kanji {
		id: 413,
		kanji: '軽',
		strokes: 12,
		meanings: &["light"],
		on_yomis: &["kei"],
		kun_yomis: &["karu-i"]
	},
	Kanji {
		id: 414,
		kanji: '農',
		strokes: 13,
		meanings: &["agriculture"],
		on_yomis: &["nō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 415,
		kanji: '返',
		strokes: 7,
		meanings: &["return"],
		on_yomis: &["hen"],
		kun_yomis: &["kae-su"]
	},
	Kanji {
		id: 416,
		kanji: '追',
		strokes: 9,
		meanings: &["follow"],
		on_yomis: &["tsui"],
		kun_yomis: &["o-u"]
	},
	Kanji {
		id: 417,
		kanji: '送',
		strokes: 9,
		meanings: &["send"],
		on_yomis: &["sō"],
		kun_yomis: &["oku-ru"]
	},
	Kanji {
		id: 418,
		kanji: '速',
		strokes: 10,
		meanings: &["fast"],
		on_yomis: &["soku"],
		kun_yomis: &["haya-i"]
	},
	Kanji {
		id: 419,
		kanji: '進',
		strokes: 11,
		meanings: &["progress"],
		on_yomis: &["shin"],
		kun_yomis: &["susu-mu"]
	},
	Kanji {
		id: 420,
		kanji: '遊',
		strokes: 12,
		meanings: &["play"],
		on_yomis: &["yū"],
		kun_yomis: &["aso-bu"]
	},
	Kanji {
		id: 421,
		kanji: '運',
		strokes: 12,
		meanings: &["carry"],
		on_yomis: &["un"],
		kun_yomis: &["hako-bu"]
	},
	Kanji {
		id: 422,
		kanji: '部',
		strokes: 11,
		meanings: &["part"],
		on_yomis: &["bu"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 423,
		kanji: '都',
		strokes: 11,
		meanings: &["metropolis"],
		on_yomis: &["to","tsu"],
		kun_yomis: &["miyako"]
	},
	Kanji {
		id: 424,
		kanji: '配',
		strokes: 10,
		meanings: &["distribute"],
		on_yomis: &["hai"],
		kun_yomis: &["kuba-ru"]
	},
	Kanji {
		id: 425,
		kanji: '酒',
		strokes: 10,
		meanings: &["liquor"],
		on_yomis: &["shu"],
		kun_yomis: &["sake","saka"]
	},
	Kanji {
		id: 426,
		kanji: '重',
		strokes: 9,
		meanings: &["heavy","gravity","pile"],
		on_yomis: &["jū","chō"],
		kun_yomis: &["omo-i","kasa-neru"]
	},
	Kanji {
		id: 427,
		kanji: '鉄',
		strokes: 13,
		meanings: &["iron"],
		on_yomis: &["tetsu"],
		kun_yomis: &["kurogane"]
	},
	Kanji {
		id: 428,
		kanji: '銀',
		strokes: 14,
		meanings: &["silver"],
		on_yomis: &["gin"],
		kun_yomis: &["shirogane"]
	},
	Kanji {
		id: 429,
		kanji: '開',
		strokes: 12,
		meanings: &["open"],
		on_yomis: &["kai"],
		kun_yomis: &["hira-ku","a-ku"]
	},
	Kanji {
		id: 430,
		kanji: '院',
		strokes: 10,
		meanings: &["institution"],
		on_yomis: &["in"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 431,
		kanji: '陽',
		strokes: 12,
		meanings: &["sun"],
		on_yomis: &["yō"],
		kun_yomis: &["hi"]
	},
	Kanji {
		id: 432,
		kanji: '階',
		strokes: 12,
		meanings: &["storey"],
		on_yomis: &["kai"],
		kun_yomis: &["kizahashi"]
	},
	Kanji {
		id: 433,
		kanji: '集',
		strokes: 12,
		meanings: &["gather"],
		on_yomis: &["shū"],
		kun_yomis: &["atsu-maru"]
	},
	Kanji {
		id: 434,
		kanji: '面',
		strokes: 9,
		meanings: &["surface"],
		on_yomis: &["men"],
		kun_yomis: &["omote","tsura"]
	},
	Kanji {
		id: 435,
		kanji: '題',
		strokes: 18,
		meanings: &["topic"],
		on_yomis: &["dai"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 436,
		kanji: '飲',
		strokes: 12,
		meanings: &["drink"],
		on_yomis: &["in"],
		kun_yomis: &["no-mu"]
	},
	Kanji {
		id: 437,
		kanji: '館',
		strokes: 16,
		meanings: &["public building"],
		on_yomis: &["kan"],
		kun_yomis: &["tate"]
	},
	Kanji {
		id: 438,
		kanji: '駅',
		strokes: 14,
		meanings: &["station"],
		on_yomis: &["eki"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 439,
		kanji: '鼻',
		strokes: 14,
		meanings: &["nose"],
		on_yomis: &["bi"],
		kun_yomis: &["hana"]
	},
	Kanji {
		id: 440,
		kanji: '不',
		strokes: 4,
		meanings: &["not"],
		on_yomis: &["fu","bu"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 441,
		kanji: '争',
		strokes: 6,
		meanings: &["conflict"],
		on_yomis: &["sō"],
		kun_yomis: &["araso-u"]
	},
	Kanji {
		id: 442,
		kanji: '付',
		strokes: 5,
		meanings: &["attach"],
		on_yomis: &["fu"],
		kun_yomis: &["tsu-ku"]
	},
	Kanji {
		id: 443,
		kanji: '令',
		strokes: 5,
		meanings: &["orders"],
		on_yomis: &["rei"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 444,
		kanji: '以',
		strokes: 5,
		meanings: &["reference point"],
		on_yomis: &["i"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 445,
		kanji: '仲',
		strokes: 6,
		meanings: &["relationship"],
		on_yomis: &["chū"],
		kun_yomis: &["naka"]
	},
	Kanji {
		id: 446,
		kanji: '伝',
		strokes: 6,
		meanings: &["convey"],
		on_yomis: &["den"],
		kun_yomis: &["tsuta-eru"]
	},
	Kanji {
		id: 447,
		kanji: '位',
		strokes: 7,
		meanings: &["rank"],
		on_yomis: &["i"],
		kun_yomis: &["kurai"]
	},
	Kanji {
		id: 448,
		kanji: '低',
		strokes: 7,
		meanings: &["low"],
		on_yomis: &["tei"],
		kun_yomis: &["hiku-i"]
	},
	Kanji {
		id: 449,
		kanji: '例',
		strokes: 8,
		meanings: &["example"],
		on_yomis: &["rei"],
		kun_yomis: &["tato-eru"]
	},
	Kanji {
		id: 450,
		kanji: '便',
		strokes: 9,
		meanings: &["facility","flight","mail"],
		on_yomis: &["ben","bin"],
		kun_yomis: &["tayo-ri"]
	},
	Kanji {
		id: 451,
		kanji: '信',
		strokes: 9,
		meanings: &["trust"],
		on_yomis: &["shin"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 452,
		kanji: '倉',
		strokes: 10,
		meanings: &["storage"],
		on_yomis: &["sō"],
		kun_yomis: &["kura"]
	},
	Kanji {
		id: 453,
		kanji: '候',
		strokes: 10,
		meanings: &["climate"],
		on_yomis: &["kō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 454,
		kanji: '借',
		strokes: 10,
		meanings: &["borrow"],
		on_yomis: &["shaku"],
		kun_yomis: &["ka-riru"]
	},
	Kanji {
		id: 455,
		kanji: '停',
		strokes: 11,
		meanings: &["halt"],
		on_yomis: &["tei"],
		kun_yomis: &["to-maru","to-meru"]
	},
	Kanji {
		id: 456,
		kanji: '健',
		strokes: 11,
		meanings: &["healthy"],
		on_yomis: &["ken"],
		kun_yomis: &["suko-yaka"]
	},
	Kanji {
		id: 457,
		kanji: '側',
		strokes: 11,
		meanings: &["side"],
		on_yomis: &["soku"],
		kun_yomis: &["kawa"]
	},
	Kanji {
		id: 458,
		kanji: '働',
		strokes: 13,
		meanings: &["work"],
		on_yomis: &["dō"],
		kun_yomis: &["hatara-ku"]
	},
	Kanji {
		id: 459,
		kanji: '億',
		strokes: 15,
		meanings: &["hundred million"],
		on_yomis: &["oku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 460,
		kanji: '兆',
		strokes: 6,
		meanings: &["portent","trillion"],
		on_yomis: &["chō"],
		kun_yomis: &["kiza-shi"]
	},
	Kanji {
		id: 461,
		kanji: '児',
		strokes: 7,
		meanings: &["offspring"],
		on_yomis: &["ji","ni"],
		kun_yomis: &["ko"]
	},
	Kanji {
		id: 462,
		kanji: '共',
		strokes: 6,
		meanings: &["together"],
		on_yomis: &["kyō"],
		kun_yomis: &["tomo"]
	},
	Kanji {
		id: 463,
		kanji: '兵',
		strokes: 7,
		meanings: &["soldier"],
		on_yomis: &["hei","hyō"],
		kun_yomis: &["tsuwamono"]
	},
	Kanji {
		id: 464,
		kanji: '典',
		strokes: 8,
		meanings: &["code"],
		on_yomis: &["ten"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 465,
		kanji: '冷',
		strokes: 7,
		meanings: &["cool"],
		on_yomis: &["rei"],
		kun_yomis: &["tsume-tai","hi-eru","sa-meru"]
	},
	Kanji {
		id: 466,
		kanji: '初',
		strokes: 7,
		meanings: &["first"],
		on_yomis: &["sho"],
		kun_yomis: &["hatsu","haji-me"]
	},
	Kanji {
		id: 467,
		kanji: '別',
		strokes: 7,
		meanings: &["separate"],
		on_yomis: &["betsu"],
		kun_yomis: &["waka-reru"]
	},
	Kanji {
		id: 468,
		kanji: '利',
		strokes: 7,
		meanings: &["profit"],
		on_yomis: &["ri"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 469,
		kanji: '刷',
		strokes: 8,
		meanings: &["printing"],
		on_yomis: &["satsu"],
		kun_yomis: &["su-ru"]
	},
	Kanji {
		id: 470,
		kanji: '副',
		strokes: 11,
		meanings: &["vice-"],
		on_yomis: &["fuku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 471,
		kanji: '功',
		strokes: 5,
		meanings: &["achievement"],
		on_yomis: &["kō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 472,
		kanji: '加',
		strokes: 5,
		meanings: &["add"],
		on_yomis: &["ka"],
		kun_yomis: &["kuwa-eru"]
	},
	Kanji {
		id: 473,
		kanji: '努',
		strokes: 7,
		meanings: &["toil"],
		on_yomis: &["do"],
		kun_yomis: &["tsuto-meru"]
	},
	Kanji {
		id: 474,
		kanji: '労',
		strokes: 7,
		meanings: &["labor"],
		on_yomis: &["rō"],
		kun_yomis: &["negira-u"]
	},
	Kanji {
		id: 475,
		kanji: '勇',
		strokes: 9,
		meanings: &["courage"],
		on_yomis: &["yū"],
		kun_yomis: &["isa-mu"]
	},
	Kanji {
		id: 476,
		kanji: '包',
		strokes: 5,
		meanings: &["wrap"],
		on_yomis: &["hō"],
		kun_yomis: &["tsutsu-mu"]
	},
	Kanji {
		id: 477,
		kanji: '卒',
		strokes: 8,
		meanings: &["graduate"],
		on_yomis: &["sotsu"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 478,
		kanji: '協',
		strokes: 8,
		meanings: &["cooperation"],
		on_yomis: &["kyō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 479,
		kanji: '単',
		strokes: 9,
		meanings: &["simple"],
		on_yomis: &["tan"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 480,
		kanji: '博',
		strokes: 12,
		meanings: &["wide knowledge","Dr."],
		on_yomis: &["haku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 481,
		kanji: '印',
		strokes: 6,
		meanings: &["mark"],
		on_yomis: &["in"],
		kun_yomis: &["shirushi"]
	},
	Kanji {
		id: 482,
		kanji: '参',
		strokes: 8,
		meanings: &["participate"],
		on_yomis: &["san"],
		kun_yomis: &["mai-ru"]
	},
	Kanji {
		id: 483,
		kanji: '史',
		strokes: 5,
		meanings: &["history"],
		on_yomis: &["shi"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 484,
		kanji: '司',
		strokes: 5,
		meanings: &["director"],
		on_yomis: &["shi"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 485,
		kanji: '各',
		strokes: 6,
		meanings: &["each"],
		on_yomis: &["kaku"],
		kun_yomis: &["ono-ono"]
	},
	Kanji {
		id: 486,
		kanji: '告',
		strokes: 7,
		meanings: &["tell"],
		on_yomis: &["koku"],
		kun_yomis: &["tsu-geru"]
	},
	Kanji {
		id: 487,
		kanji: '周',
		strokes: 8,
		meanings: &["circumference"],
		on_yomis: &["shū"],
		kun_yomis: &["mawa-ri"]
	},
	Kanji {
		id: 488,
		kanji: '唱',
		strokes: 11,
		meanings: &["chant"],
		on_yomis: &["shō"],
		kun_yomis: &["tona-eru"]
	},
	Kanji {
		id: 489,
		kanji: '喜',
		strokes: 12,
		meanings: &["rejoice","joy"],
		on_yomis: &["ki"],
		kun_yomis: &["yoroko-bu"]
	},
	Kanji {
		id: 490,
		kanji: '器',
		strokes: 15,
		meanings: &["container"],
		on_yomis: &["ki"],
		kun_yomis: &["utsuwa"]
	},
	Kanji {
		id: 491,
		kanji: '囲',
		strokes: 7,
		meanings: &["surround"],
		on_yomis: &["i"],
		kun_yomis: &["kako-u"]
	},
	Kanji {
		id: 492,
		kanji: '固',
		strokes: 8,
		meanings: &["harden"],
		on_yomis: &["ko"],
		kun_yomis: &["kata-maru"]
	},
	Kanji {
		id: 493,
		kanji: '型',
		strokes: 9,
		meanings: &["model"],
		on_yomis: &["kei"],
		kun_yomis: &["kata"]
	},
	Kanji {
		id: 494,
		kanji: '堂',
		strokes: 11,
		meanings: &["public chamber"],
		on_yomis: &["dō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 495,
		kanji: '塩',
		strokes: 13,
		meanings: &["salt"],
		on_yomis: &["en"],
		kun_yomis: &["shio"]
	},
	Kanji {
		id: 496,
		kanji: '士',
		strokes: 3,
		meanings: &["gentleman"],
		on_yomis: &["shi"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 497,
		kanji: '変',
		strokes: 9,
		meanings: &["change","strange"],
		on_yomis: &["hen"],
		kun_yomis: &["ka-waru"]
	},
	Kanji {
		id: 498,
		kanji: '夫',
		strokes: 4,
		meanings: &["husband"],
		on_yomis: &["fu fū bu"],
		kun_yomis: &["otto"]
	},
	Kanji {
		id: 499,
		kanji: '失',
		strokes: 5,
		meanings: &["lose"],
		on_yomis: &["shitsu"],
		kun_yomis: &["ushina-u"]
	},
	Kanji {
		id: 500,
		kanji: '好',
		strokes: 6,
		meanings: &["like"],
		on_yomis: &["kō"],
		kun_yomis: &["su-ku","kono-mu"]
	},
	Kanji {
		id: 501,
		kanji: '季',
		strokes: 8,
		meanings: &["seasons"],
		on_yomis: &["ki"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 502,
		kanji: '孫',
		strokes: 10,
		meanings: &["grandchild"],
		on_yomis: &["son"],
		kun_yomis: &["mago"]
	},
	Kanji {
		id: 503,
		kanji: '完',
		strokes: 7,
		meanings: &["perfect"],
		on_yomis: &["kan"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 504,
		kanji: '官',
		strokes: 8,
		meanings: &["government official"],
		on_yomis: &["kan"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 505,
		kanji: '害',
		strokes: 10,
		meanings: &["harm"],
		on_yomis: &["gai"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 506,
		kanji: '察',
		strokes: 14,
		meanings: &["guess"],
		on_yomis: &["satsu"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 507,
		kanji: '巣',
		strokes: 11,
		meanings: &["nest"],
		on_yomis: &["sō"],
		kun_yomis: &["su"]
	},
	Kanji {
		id: 508,
		kanji: '差',
		strokes: 10,
		meanings: &["distinction"],
		on_yomis: &["sa"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 509,
		kanji: '希',
		strokes: 7,
		meanings: &["hope"],
		on_yomis: &["ki"],
		kun_yomis: &["mare"]
	},
	Kanji {
		id: 510,
		kanji: '席',
		strokes: 10,
		meanings: &["seat"],
		on_yomis: &["seki"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 511,
		kanji: '帯',
		strokes: 10,
		meanings: &["sash"],
		on_yomis: &["tai"],
		kun_yomis: &["obi"]
	},
	Kanji {
		id: 512,
		kanji: '底',
		strokes: 8,
		meanings: &["bottom"],
		on_yomis: &["tei"],
		kun_yomis: &["soko"]
	},
	Kanji {
		id: 513,
		kanji: '府',
		strokes: 8,
		meanings: &["urban prefecture"],
		on_yomis: &["fu"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 514,
		kanji: '康',
		strokes: 11,
		meanings: &["ease"],
		on_yomis: &["kō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 515,
		kanji: '建',
		strokes: 9,
		meanings: &["build"],
		on_yomis: &["ken"],
		kun_yomis: &["ta-teru"]
	},
	Kanji {
		id: 516,
		kanji: '径',
		strokes: 8,
		meanings: &["diameter"],
		on_yomis: &["kei"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 517,
		kanji: '徒',
		strokes: 10,
		meanings: &["junior"],
		on_yomis: &["to"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 518,
		kanji: '得',
		strokes: 11,
		meanings: &["acquire"],
		on_yomis: &["toku"],
		kun_yomis: &["e-ru"]
	},
	Kanji {
		id: 519,
		kanji: '必',
		strokes: 5,
		meanings: &["inevitable"],
		on_yomis: &["hitsu"],
		kun_yomis: &["kanara-zu"]
	},
	Kanji {
		id: 520,
		kanji: '念',
		strokes: 8,
		meanings: &["thought"],
		on_yomis: &["nen"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 521,
		kanji: '愛',
		strokes: 13,
		meanings: &["love"],
		on_yomis: &["ai"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 522,
		kanji: '成',
		strokes: 6,
		meanings: &["become"],
		on_yomis: &["sei"],
		kun_yomis: &["na-ru"]
	},
	Kanji {
		id: 523,
		kanji: '戦',
		strokes: 13,
		meanings: &["war"],
		on_yomis: &["sen"],
		kun_yomis: &["ikusa","tataka-u"]
	},
	Kanji {
		id: 524,
		kanji: '折',
		strokes: 7,
		meanings: &["fold","break"],
		on_yomis: &["setsu"],
		kun_yomis: &["o-ru"]
	},
	Kanji {
		id: 525,
		kanji: '挙',
		strokes: 10,
		meanings: &["raise"],
		on_yomis: &["kyo"],
		kun_yomis: &["a-geru"]
	},
	Kanji {
		id: 526,
		kanji: '改',
		strokes: 7,
		meanings: &["reformation"],
		on_yomis: &["kai"],
		kun_yomis: &["arata-meru"]
	},
	Kanji {
		id: 527,
		kanji: '救',
		strokes: 11,
		meanings: &["salvation"],
		on_yomis: &["kyū"],
		kun_yomis: &["suku-u"]
	},
	Kanji {
		id: 528,
		kanji: '敗',
		strokes: 11,
		meanings: &["break","failure"],
		on_yomis: &["hai"],
		kun_yomis: &["yabu-reru"]
	},
	Kanji {
		id: 529,
		kanji: '散',
		strokes: 12,
		meanings: &["scatter"],
		on_yomis: &["san"],
		kun_yomis: &["chi-ru"]
	},
	Kanji {
		id: 530,
		kanji: '料',
		strokes: 10,
		meanings: &["fee"],
		on_yomis: &["ryō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 531,
		kanji: '旗',
		strokes: 14,
		meanings: &["flag"],
		on_yomis: &["ki"],
		kun_yomis: &["hata"]
	},
	Kanji {
		id: 532,
		kanji: '昨',
		strokes: 9,
		meanings: &["yesterday"],
		on_yomis: &["saku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 533,
		kanji: '景',
		strokes: 12,
		meanings: &["scenery"],
		on_yomis: &["kei"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 534,
		kanji: '最',
		strokes: 12,
		meanings: &["superlative"],
		on_yomis: &["sai"],
		kun_yomis: &["mo","motto-mo"]
	},
	Kanji {
		id: 535,
		kanji: '望',
		strokes: 11,
		meanings: &["hope"],
		on_yomis: &["bō"],
		kun_yomis: &["nozo-mu"]
	},
	Kanji {
		id: 536,
		kanji: '未',
		strokes: 5,
		meanings: &["un-"],
		on_yomis: &["mi"],
		kun_yomis: &["ima-da"]
	},
	Kanji {
		id: 537,
		kanji: '末',
		strokes: 5,
		meanings: &["end"],
		on_yomis: &["matsu"],
		kun_yomis: &["sue"]
	},
	Kanji {
		id: 538,
		kanji: '札',
		strokes: 5,
		meanings: &["bill","plate","tag"],
		on_yomis: &["satsu"],
		kun_yomis: &["fuda"]
	},
	Kanji {
		id: 539,
		kanji: '材',
		strokes: 7,
		meanings: &["lumber","material"],
		on_yomis: &["zai"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 540,
		kanji: '束',
		strokes: 7,
		meanings: &["bundle"],
		on_yomis: &["soku"],
		kun_yomis: &["taba","tsuka"]
	},
	Kanji {
		id: 541,
		kanji: '松',
		strokes: 8,
		meanings: &["pine"],
		on_yomis: &["shō"],
		kun_yomis: &["matsu"]
	},
	Kanji {
		id: 542,
		kanji: '果',
		strokes: 8,
		meanings: &["accomplish","fruit"],
		on_yomis: &["ka"],
		kun_yomis: &["ha-tasu"]
	},
	Kanji {
		id: 543,
		kanji: '栄',
		strokes: 9,
		meanings: &["prosperity"],
		on_yomis: &["ei"],
		kun_yomis: &["saka-eru"]
	},
	Kanji {
		id: 544,
		kanji: '案',
		strokes: 10,
		meanings: &["plan"],
		on_yomis: &["an"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 545,
		kanji: '梅',
		strokes: 10,
		meanings: &["plum"],
		on_yomis: &["bai"],
		kun_yomis: &["ume"]
	},
	Kanji {
		id: 546,
		kanji: '械',
		strokes: 11,
		meanings: &["contraption"],
		on_yomis: &["kai"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 547,
		kanji: '極',
		strokes: 12,
		meanings: &["poles"],
		on_yomis: &["kyoku"],
		kun_yomis: &["kiwa-meru"]
	},
	Kanji {
		id: 548,
		kanji: '標',
		strokes: 15,
		meanings: &["signpost"],
		on_yomis: &["hyō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 549,
		kanji: '機',
		strokes: 16,
		meanings: &["machine"],
		on_yomis: &["ki"],
		kun_yomis: &["hata"]
	},
	Kanji {
		id: 550,
		kanji: '欠',
		strokes: 4,
		meanings: &["lack"],
		on_yomis: &["ketsu"],
		kun_yomis: &["ka-keru"]
	},
	Kanji {
		id: 551,
		kanji: '歴',
		strokes: 14,
		meanings: &["curriculum"],
		on_yomis: &["reki"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 552,
		kanji: '残',
		strokes: 10,
		meanings: &["remainder","left"],
		on_yomis: &["zan"],
		kun_yomis: &["noko-ru"]
	},
	Kanji {
		id: 553,
		kanji: '殺',
		strokes: 10,
		meanings: &["kill"],
		on_yomis: &["satsu"],
		kun_yomis: &["koro-su"]
	},
	Kanji {
		id: 554,
		kanji: '毒',
		strokes: 8,
		meanings: &["poison"],
		on_yomis: &["doku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 555,
		kanji: '氏',
		strokes: 4,
		meanings: &["surname","Mr."],
		on_yomis: &["shi"],
		kun_yomis: &["uji"]
	},
	Kanji {
		id: 556,
		kanji: '民',
		strokes: 5,
		meanings: &["people"],
		on_yomis: &["min"],
		kun_yomis: &["tami"]
	},
	Kanji {
		id: 557,
		kanji: '求',
		strokes: 7,
		meanings: &["request"],
		on_yomis: &["kyū"],
		kun_yomis: &["moto-mu"]
	},
	Kanji {
		id: 558,
		kanji: '治',
		strokes: 8,
		meanings: &["govern","heal"],
		on_yomis: &["chi","ji"],
		kun_yomis: &["osa-meru","nao-ru"]
	},
	Kanji {
		id: 559,
		kanji: '法',
		strokes: 8,
		meanings: &["method"],
		on_yomis: &["hō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 560,
		kanji: '泣',
		strokes: 8,
		meanings: &["cry"],
		on_yomis: &["kyū"],
		kun_yomis: &["na-ku"]
	},
	Kanji {
		id: 561,
		kanji: '浅',
		strokes: 9,
		meanings: &["shallow"],
		on_yomis: &["sen"],
		kun_yomis: &["asa-i"]
	},
	Kanji {
		id: 562,
		kanji: '浴',
		strokes: 10,
		meanings: &["bathe","bask"],
		on_yomis: &["yoku"],
		kun_yomis: &["a-biru"]
	},
	Kanji {
		id: 563,
		kanji: '清',
		strokes: 11,
		meanings: &["pure"],
		on_yomis: &["sei","shō"],
		kun_yomis: &["kiyo-raka"]
	},
	Kanji {
		id: 564,
		kanji: '満',
		strokes: 12,
		meanings: &["full"],
		on_yomis: &["man"],
		kun_yomis: &["mi-chiru"]
	},
	Kanji {
		id: 565,
		kanji: '漁',
		strokes: 14,
		meanings: &["look for","fishing"],
		on_yomis: &["ryō","gyo"],
		kun_yomis: &["asa-ru"]
	},
	Kanji {
		id: 566,
		kanji: '灯',
		strokes: 6,
		meanings: &["lamp"],
		on_yomis: &["tō"],
		kun_yomis: &["hi"]
	},
	Kanji {
		id: 567,
		kanji: '無',
		strokes: 12,
		meanings: &["nothing"],
		on_yomis: &["mu","bu"],
		kun_yomis: &["na-i"]
	},
	Kanji {
		id: 568,
		kanji: '然',
		strokes: 12,
		meanings: &["so","although"],
		on_yomis: &["zen","nen"],
		kun_yomis: &["shika-shi"]
	},
	Kanji {
		id: 569,
		kanji: '焼',
		strokes: 12,
		meanings: &["bake"],
		on_yomis: &["shō"],
		kun_yomis: &["ya-ku"]
	},
	Kanji {
		id: 570,
		kanji: '照',
		strokes: 13,
		meanings: &["illuminate"],
		on_yomis: &["shō"],
		kun_yomis: &["te-rasu"]
	},
	Kanji {
		id: 571,
		kanji: '熱',
		strokes: 15,
		meanings: &["heat"],
		on_yomis: &["netsu"],
		kun_yomis: &["atsu-i"]
	},
	Kanji {
		id: 572,
		kanji: '牧',
		strokes: 8,
		meanings: &["pasture","breed"],
		on_yomis: &["boku"],
		kun_yomis: &["maki"]
	},
	Kanji {
		id: 573,
		kanji: '特',
		strokes: 10,
		meanings: &["special"],
		on_yomis: &["toku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 574,
		kanji: '産',
		strokes: 11,
		meanings: &["give birth"],
		on_yomis: &["san"],
		kun_yomis: &["u-mu"]
	},
	Kanji {
		id: 575,
		kanji: '的',
		strokes: 8,
		meanings: &["target"],
		on_yomis: &["teki"],
		kun_yomis: &["mato"]
	},
	Kanji {
		id: 576,
		kanji: '省',
		strokes: 9,
		meanings: &["government ministry","omit","look back"],
		on_yomis: &["shō","sei"],
		kun_yomis: &["habu-ku"]
	},
	Kanji {
		id: 577,
		kanji: '祝',
		strokes: 9,
		meanings: &["celebrate"],
		on_yomis: &["shuku"],
		kun_yomis: &["iwa-u"]
	},
	Kanji {
		id: 578,
		kanji: '票',
		strokes: 11,
		meanings: &["ballot"],
		on_yomis: &["hyō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 579,
		kanji: '種',
		strokes: 14,
		meanings: &["species","seed"],
		on_yomis: &["shu"],
		kun_yomis: &["tane"]
	},
	Kanji {
		id: 580,
		kanji: '積',
		strokes: 16,
		meanings: &["accumulate","pile"],
		on_yomis: &["seki"],
		kun_yomis: &["tsu-mu"]
	},
	Kanji {
		id: 581,
		kanji: '競',
		strokes: 20,
		meanings: &["emulate"],
		on_yomis: &["kyō"],
		kun_yomis: &["kiso-u"]
	},
	Kanji {
		id: 582,
		kanji: '笑',
		strokes: 10,
		meanings: &["laugh"],
		on_yomis: &["shō"],
		kun_yomis: &["wara-u"]
	},
	Kanji {
		id: 583,
		kanji: '管',
		strokes: 14,
		meanings: &["pipe"],
		on_yomis: &["kan"],
		kun_yomis: &["kuda"]
	},
	Kanji {
		id: 584,
		kanji: '節',
		strokes: 13,
		meanings: &["node"],
		on_yomis: &["setsu"],
		kun_yomis: &["fushi"]
	},
	Kanji {
		id: 585,
		kanji: '粉',
		strokes: 10,
		meanings: &["flour"],
		on_yomis: &["fun"],
		kun_yomis: &["ko","kona"]
	},
	Kanji {
		id: 586,
		kanji: '紀',
		strokes: 9,
		meanings: &["chronicle"],
		on_yomis: &["ki"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 587,
		kanji: '約',
		strokes: 9,
		meanings: &["promise"],
		on_yomis: &["yaku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 588,
		kanji: '結',
		strokes: 12,
		meanings: &["tie"],
		on_yomis: &["ketsu"],
		kun_yomis: &["musu-bu","yu-u"]
	},
	Kanji {
		id: 589,
		kanji: '給',
		strokes: 12,
		meanings: &["salary"],
		on_yomis: &["kyū"],
		kun_yomis: &["tama-u"]
	},
	Kanji {
		id: 590,
		kanji: '続',
		strokes: 13,
		meanings: &["continue"],
		on_yomis: &["zoku"],
		kun_yomis: &["tsuzu-ku"]
	},
	Kanji {
		id: 591,
		kanji: '置',
		strokes: 13,
		meanings: &["put"],
		on_yomis: &["chi"],
		kun_yomis: &["o-ku"]
	},
	Kanji {
		id: 592,
		kanji: '老',
		strokes: 6,
		meanings: &["old man"],
		on_yomis: &["rō"],
		kun_yomis: &["o-iru"]
	},
	Kanji {
		id: 593,
		kanji: '胃',
		strokes: 9,
		meanings: &["stomach"],
		on_yomis: &["i"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 594,
		kanji: '脈',
		strokes: 10,
		meanings: &["vein"],
		on_yomis: &["myaku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 595,
		kanji: '腸',
		strokes: 13,
		meanings: &["intestines"],
		on_yomis: &["chō"],
		kun_yomis: &["harawata"]
	},
	Kanji {
		id: 596,
		kanji: '臣',
		strokes: 7,
		meanings: &["retainer"],
		on_yomis: &["shin"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 597,
		kanji: '航',
		strokes: 10,
		meanings: &["cruise"],
		on_yomis: &["kō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 598,
		kanji: '良',
		strokes: 7,
		meanings: &["good"],
		on_yomis: &["ryō"],
		kun_yomis: &["yo-i"]
	},
	Kanji {
		id: 599,
		kanji: '芸',
		strokes: 7,
		meanings: &["art"],
		on_yomis: &["gei"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 600,
		kanji: '芽',
		strokes: 8,
		meanings: &["bud"],
		on_yomis: &["ga"],
		kun_yomis: &["me"]
	},
	Kanji {
		id: 601,
		kanji: '英',
		strokes: 8,
		meanings: &["England"],
		on_yomis: &["ei"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 602,
		kanji: '菜',
		strokes: 11,
		meanings: &["vegetable"],
		on_yomis: &["sai"],
		kun_yomis: &["na"]
	},
	Kanji {
		id: 603,
		kanji: '街',
		strokes: 12,
		meanings: &["street","city"],
		on_yomis: &["gai"],
		kun_yomis: &["machi"]
	},
	Kanji {
		id: 604,
		kanji: '衣',
		strokes: 6,
		meanings: &["garment"],
		on_yomis: &["i"],
		kun_yomis: &["koromo"]
	},
	Kanji {
		id: 605,
		kanji: '要',
		strokes: 9,
		meanings: &["need"],
		on_yomis: &["yō"],
		kun_yomis: &["i-ru"]
	},
	Kanji {
		id: 606,
		kanji: '覚',
		strokes: 12,
		meanings: &["memorize"],
		on_yomis: &["kaku"],
		kun_yomis: &["obo-eru","sa-meru"]
	},
	Kanji {
		id: 607,
		kanji: '観',
		strokes: 18,
		meanings: &["observe"],
		on_yomis: &["kan"],
		kun_yomis: &["mi-ru"]
	},
	Kanji {
		id: 608,
		kanji: '訓',
		strokes: 10,
		meanings: &["instruction"],
		on_yomis: &["kun"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 609,
		kanji: '試',
		strokes: 13,
		meanings: &["test"],
		on_yomis: &["shi"],
		kun_yomis: &["kokoro-miru","tame-su"]
	},
	Kanji {
		id: 610,
		kanji: '説',
		strokes: 14,
		meanings: &["theory"],
		on_yomis: &["setsu"],
		kun_yomis: &["to-ku"]
	},
	Kanji {
		id: 611,
		kanji: '課',
		strokes: 15,
		meanings: &["section"],
		on_yomis: &["ka"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 612,
		kanji: '議',
		strokes: 20,
		meanings: &["deliberation"],
		on_yomis: &["gi"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 613,
		kanji: '象',
		strokes: 12,
		meanings: &["elephant","figure"],
		on_yomis: &["zō","shō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 614,
		kanji: '貨',
		strokes: 11,
		meanings: &["currency","cargo"],
		on_yomis: &["ka"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 615,
		kanji: '貯',
		strokes: 12,
		meanings: &["savings"],
		on_yomis: &["cho"],
		kun_yomis: &["ta-meru"]
	},
	Kanji {
		id: 616,
		kanji: '費',
		strokes: 12,
		meanings: &["expense"],
		on_yomis: &["hi"],
		kun_yomis: &["tsui-yasu"]
	},
	Kanji {
		id: 617,
		kanji: '賞',
		strokes: 15,
		meanings: &["prize"],
		on_yomis: &["shō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 618,
		kanji: '軍',
		strokes: 9,
		meanings: &["army"],
		on_yomis: &["gun"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 619,
		kanji: '輪',
		strokes: 15,
		meanings: &["wheel"],
		on_yomis: &["rin"],
		kun_yomis: &["wa"]
	},
	Kanji {
		id: 620,
		kanji: '辞',
		strokes: 13,
		meanings: &["resign","speech","encyclopedia"],
		on_yomis: &["ji"],
		kun_yomis: &["kotoba","ya-meru"]
	},
	Kanji {
		id: 621,
		kanji: '辺',
		strokes: 5,
		meanings: &["edge","vicinity"],
		on_yomis: &["hen"],
		kun_yomis: &["ata-ri"]
	},
	Kanji {
		id: 622,
		kanji: '連',
		strokes: 10,
		meanings: &["take along"],
		on_yomis: &["ren"],
		kun_yomis: &["tsu-reru","tsura-neru"]
	},
	Kanji {
		id: 623,
		kanji: '達',
		strokes: 12,
		meanings: &["attain"],
		on_yomis: &["tatsu"],
		kun_yomis: &["tachi"]
	},
	Kanji {
		id: 624,
		kanji: '選',
		strokes: 15,
		meanings: &["choose"],
		on_yomis: &["sen"],
		kun_yomis: &["era-bu"]
	},
	Kanji {
		id: 625,
		kanji: '郡',
		strokes: 10,
		meanings: &["county"],
		on_yomis: &["gun"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 626,
		kanji: '量',
		strokes: 12,
		meanings: &["quantity"],
		on_yomis: &["ryō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 627,
		kanji: '録',
		strokes: 16,
		meanings: &["transcript"],
		on_yomis: &["roku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 628,
		kanji: '鏡',
		strokes: 19,
		meanings: &["mirror"],
		on_yomis: &["kyō"],
		kun_yomis: &["kagami"]
	},
	Kanji {
		id: 629,
		kanji: '関',
		strokes: 14,
		meanings: &["related"],
		on_yomis: &["kan"],
		kun_yomis: &["seki"]
	},
	Kanji {
		id: 630,
		kanji: '陸',
		strokes: 11,
		meanings: &["land"],
		on_yomis: &["riku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 631,
		kanji: '隊',
		strokes: 12,
		meanings: &["squad"],
		on_yomis: &["tai"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 632,
		kanji: '静',
		strokes: 14,
		meanings: &["quiet"],
		on_yomis: &["sei"],
		kun_yomis: &["shizu-ka"]
	},
	Kanji {
		id: 633,
		kanji: '順',
		strokes: 12,
		meanings: &["obey"],
		on_yomis: &["jun"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 634,
		kanji: '願',
		strokes: 19,
		meanings: &["request"],
		on_yomis: &["gan"],
		kun_yomis: &["nega-u"]
	},
	Kanji {
		id: 635,
		kanji: '類',
		strokes: 18,
		meanings: &["sort"],
		on_yomis: &["rui"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 636,
		kanji: '飛',
		strokes: 9,
		meanings: &["fly"],
		on_yomis: &["hi"],
		kun_yomis: &["to-bu"]
	},
	Kanji {
		id: 637,
		kanji: '飯',
		strokes: 12,
		meanings: &["meal"],
		on_yomis: &["han"],
		kun_yomis: &["meshi"]
	},
	Kanji {
		id: 638,
		kanji: '養',
		strokes: 15,
		meanings: &["foster"],
		on_yomis: &["yō"],
		kun_yomis: &["yashina-u"]
	},
	Kanji {
		id: 639,
		kanji: '験',
		strokes: 18,
		meanings: &["verify"],
		on_yomis: &["ken"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 640,
		kanji: '久',
		strokes: 3,
		meanings: &["long time"],
		on_yomis: &["kyū"],
		kun_yomis: &["hisa"]
	},
	Kanji {
		id: 641,
		kanji: '仏',
		strokes: 4,
		meanings: &["Buddha"],
		on_yomis: &["futsu","butsu"],
		kun_yomis: &["hotoke"]
	},
	Kanji {
		id: 642,
		kanji: '仮',
		strokes: 6,
		meanings: &["sham"],
		on_yomis: &["ka","ke"],
		kun_yomis: &["kari"]
	},
	Kanji {
		id: 643,
		kanji: '件',
		strokes: 6,
		meanings: &["affair"],
		on_yomis: &["ken"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 644,
		kanji: '任',
		strokes: 6,
		meanings: &["responsibility"],
		on_yomis: &["nin"],
		kun_yomis: &["maka-seru"]
	},
	Kanji {
		id: 645,
		kanji: '似',
		strokes: 7,
		meanings: &["resemble"],
		on_yomis: &["ji"],
		kun_yomis: &["ni-ru"]
	},
	Kanji {
		id: 646,
		kanji: '余',
		strokes: 7,
		meanings: &["surplus"],
		on_yomis: &["yo"],
		kun_yomis: &["ama-ru"]
	},
	Kanji {
		id: 647,
		kanji: '価',
		strokes: 8,
		meanings: &["value"],
		on_yomis: &["ka"],
		kun_yomis: &["atai"]
	},
	Kanji {
		id: 648,
		kanji: '保',
		strokes: 9,
		meanings: &["preserve"],
		on_yomis: &["ho"],
		kun_yomis: &["tamo-tsu"]
	},
	Kanji {
		id: 649,
		kanji: '修',
		strokes: 10,
		meanings: &["discipline"],
		on_yomis: &["shū"],
		kun_yomis: &["osa-meru"]
	},
	Kanji {
		id: 650,
		kanji: '俵',
		strokes: 10,
		meanings: &["straw bag"],
		on_yomis: &["hyō"],
		kun_yomis: &["tawara"]
	},
	Kanji {
		id: 651,
		kanji: '個',
		strokes: 10,
		meanings: &["individual"],
		on_yomis: &["ko"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 652,
		kanji: '備',
		strokes: 12,
		meanings: &["provide"],
		on_yomis: &["bi"],
		kun_yomis: &["sona-eru"]
	},
	Kanji {
		id: 653,
		kanji: '像',
		strokes: 14,
		meanings: &["statue"],
		on_yomis: &["zō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 654,
		kanji: '再',
		strokes: 6,
		meanings: &["again"],
		on_yomis: &["sai","sa"],
		kun_yomis: &["futata-bi"]
	},
	Kanji {
		id: 655,
		kanji: '刊',
		strokes: 5,
		meanings: &["publish"],
		on_yomis: &["kan"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 656,
		kanji: '判',
		strokes: 7,
		meanings: &["judge"],
		on_yomis: &["han"],
		kun_yomis: &["waka-ru"]
	},
	Kanji {
		id: 657,
		kanji: '制',
		strokes: 8,
		meanings: &["control"],
		on_yomis: &["sei"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 658,
		kanji: '券',
		strokes: 8,
		meanings: &["ticket"],
		on_yomis: &["ken"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 659,
		kanji: '則',
		strokes: 9,
		meanings: &["rule"],
		on_yomis: &["soku"],
		kun_yomis: &["notto-ru"]
	},
	Kanji {
		id: 660,
		kanji: '効',
		strokes: 8,
		meanings: &["effect"],
		on_yomis: &["kō"],
		kun_yomis: &["ki-ku"]
	},
	Kanji {
		id: 661,
		kanji: '務',
		strokes: 11,
		meanings: &["duty"],
		on_yomis: &["mu"],
		kun_yomis: &["tsuto-meru"]
	},
	Kanji {
		id: 662,
		kanji: '勢',
		strokes: 13,
		meanings: &["energy"],
		on_yomis: &["sei"],
		kun_yomis: &["ikio-i"]
	},
	Kanji {
		id: 663,
		kanji: '厚',
		strokes: 9,
		meanings: &["thick"],
		on_yomis: &["kō"],
		kun_yomis: &["atsu-i"]
	},
	Kanji {
		id: 664,
		kanji: '句',
		strokes: 5,
		meanings: &["phrase"],
		on_yomis: &["ku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 665,
		kanji: '可',
		strokes: 5,
		meanings: &["possible"],
		on_yomis: &["ka"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 666,
		kanji: '営',
		strokes: 12,
		meanings: &["manage"],
		on_yomis: &["ei"],
		kun_yomis: &["itona-mu"]
	},
	Kanji {
		id: 667,
		kanji: '因',
		strokes: 6,
		meanings: &["cause"],
		on_yomis: &["in"],
		kun_yomis: &["yo-ru"]
	},
	Kanji {
		id: 668,
		kanji: '団',
		strokes: 6,
		meanings: &["association"],
		on_yomis: &["dan","ton"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 669,
		kanji: '圧',
		strokes: 5,
		meanings: &["pressure"],
		on_yomis: &["atsu"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 670,
		kanji: '在',
		strokes: 6,
		meanings: &["exist"],
		on_yomis: &["zai"],
		kun_yomis: &["a-ru"]
	},
	Kanji {
		id: 671,
		kanji: '均',
		strokes: 7,
		meanings: &["level"],
		on_yomis: &["kin"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 672,
		kanji: '基',
		strokes: 11,
		meanings: &["foundation"],
		on_yomis: &["ki"],
		kun_yomis: &["moto-zuku"]
	},
	Kanji {
		id: 673,
		kanji: '報',
		strokes: 12,
		meanings: &["report"],
		on_yomis: &["hō"],
		kun_yomis: &["muku-iru"]
	},
	Kanji {
		id: 674,
		kanji: '境',
		strokes: 14,
		meanings: &["boundary"],
		on_yomis: &["kyō"],
		kun_yomis: &["sakai"]
	},
	Kanji {
		id: 675,
		kanji: '墓',
		strokes: 13,
		meanings: &["grave"],
		on_yomis: &["bo"],
		kun_yomis: &["haka"]
	},
	Kanji {
		id: 676,
		kanji: '増',
		strokes: 14,
		meanings: &["increase"],
		on_yomis: &["zō"],
		kun_yomis: &["ma-su","fu-eru"]
	},
	Kanji {
		id: 677,
		kanji: '夢',
		strokes: 13,
		meanings: &["dream"],
		on_yomis: &["mu"],
		kun_yomis: &["yume"]
	},
	Kanji {
		id: 678,
		kanji: '妻',
		strokes: 8,
		meanings: &["wife"],
		on_yomis: &["sai"],
		kun_yomis: &["tsuma"]
	},
	Kanji {
		id: 679,
		kanji: '婦',
		strokes: 11,
		meanings: &["lady"],
		on_yomis: &["fu"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 680,
		kanji: '容',
		strokes: 10,
		meanings: &["contain"],
		on_yomis: &["yō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 681,
		kanji: '寄',
		strokes: 11,
		meanings: &["approach"],
		on_yomis: &["ki"],
		kun_yomis: &["yo-ru"]
	},
	Kanji {
		id: 682,
		kanji: '富',
		strokes: 12,
		meanings: &["abundant"],
		on_yomis: &["fu"],
		kun_yomis: &["tomi"]
	},
	Kanji {
		id: 683,
		kanji: '導',
		strokes: 15,
		meanings: &["guide"],
		on_yomis: &["dō"],
		kun_yomis: &["michibi-ku"]
	},
	Kanji {
		id: 684,
		kanji: '居',
		strokes: 8,
		meanings: &["reside"],
		on_yomis: &["kyo"],
		kun_yomis: &["i-ru"]
	},
	Kanji {
		id: 685,
		kanji: '属',
		strokes: 12,
		meanings: &["belong"],
		on_yomis: &["zoku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 686,
		kanji: '布',
		strokes: 5,
		meanings: &["linen"],
		on_yomis: &["fu"],
		kun_yomis: &["nuno"]
	},
	Kanji {
		id: 687,
		kanji: '師',
		strokes: 10,
		meanings: &["expert"],
		on_yomis: &["shi"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 688,
		kanji: '常',
		strokes: 11,
		meanings: &["normal"],
		on_yomis: &["jō"],
		kun_yomis: &["tsune"]
	},
	Kanji {
		id: 689,
		kanji: '幹',
		strokes: 13,
		meanings: &["tree trunk"],
		on_yomis: &["kan"],
		kun_yomis: &["miki"]
	},
	Kanji {
		id: 690,
		kanji: '序',
		strokes: 7,
		meanings: &["preface"],
		on_yomis: &["jo"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 691,
		kanji: '弁',
		strokes: 5,
		meanings: &["valve"],
		on_yomis: &["ben"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 692,
		kanji: '張',
		strokes: 11,
		meanings: &["stretch"],
		on_yomis: &["chō"],
		kun_yomis: &["ha-ru"]
	},
	Kanji {
		id: 693,
		kanji: '往',
		strokes: 8,
		meanings: &["journey"],
		on_yomis: &["ō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 694,
		kanji: '復',
		strokes: 12,
		meanings: &["recovery"],
		on_yomis: &["fuku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 695,
		kanji: '徳',
		strokes: 14,
		meanings: &["virtue"],
		on_yomis: &["toku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 696,
		kanji: '志',
		strokes: 7,
		meanings: &["intention"],
		on_yomis: &["shi"],
		kun_yomis: &["kokorozashi"]
	},
	Kanji {
		id: 697,
		kanji: '応',
		strokes: 7,
		meanings: &["respond"],
		on_yomis: &["ō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 698,
		kanji: '快',
		strokes: 7,
		meanings: &["cheerful"],
		on_yomis: &["kai"],
		kun_yomis: &["kokoroyo-i"]
	},
	Kanji {
		id: 699,
		kanji: '性',
		strokes: 8,
		meanings: &["gender"],
		on_yomis: &["sei","shō"],
		kun_yomis: &["saga"]
	},
	Kanji {
		id: 700,
		kanji: '恩',
		strokes: 10,
		meanings: &["grace"],
		on_yomis: &["on"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 701,
		kanji: '情',
		strokes: 11,
		meanings: &["feelings"],
		on_yomis: &["jō"],
		kun_yomis: &["nasa-ke"]
	},
	Kanji {
		id: 702,
		kanji: '態',
		strokes: 14,
		meanings: &["condition"],
		on_yomis: &["tai"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 703,
		kanji: '慣',
		strokes: 14,
		meanings: &["accustomed"],
		on_yomis: &["kan"],
		kun_yomis: &["na-reru"]
	},
	Kanji {
		id: 704,
		kanji: '承',
		strokes: 8,
		meanings: &["acquiesce"],
		on_yomis: &["shō","jō"],
		kun_yomis: &["uketamawa-ru"]
	},
	Kanji {
		id: 705,
		kanji: '技',
		strokes: 7,
		meanings: &["skill"],
		on_yomis: &["gi"],
		kun_yomis: &["waza"]
	},
	Kanji {
		id: 706,
		kanji: '招',
		strokes: 8,
		meanings: &["beckon"],
		on_yomis: &["shō"],
		kun_yomis: &["mane-ku"]
	},
	Kanji {
		id: 707,
		kanji: '授',
		strokes: 11,
		meanings: &["instruct"],
		on_yomis: &["ju"],
		kun_yomis: &["sazu-keru"]
	},
	Kanji {
		id: 708,
		kanji: '採',
		strokes: 11,
		meanings: &["pick"],
		on_yomis: &["sai"],
		kun_yomis: &["to-ru"]
	},
	Kanji {
		id: 709,
		kanji: '接',
		strokes: 11,
		meanings: &["contact"],
		on_yomis: &["setsu"],
		kun_yomis: &["tsu-gu"]
	},
	Kanji {
		id: 710,
		kanji: '提',
		strokes: 12,
		meanings: &["propose"],
		on_yomis: &["tei"],
		kun_yomis: &["sa-geru"]
	},
	Kanji {
		id: 711,
		kanji: '損',
		strokes: 13,
		meanings: &["loss"],
		on_yomis: &["son"],
		kun_yomis: &["soko-neru"]
	},
	Kanji {
		id: 712,
		kanji: '支',
		strokes: 4,
		meanings: &["support"],
		on_yomis: &["shi"],
		kun_yomis: &["sasa-eru"]
	},
	Kanji {
		id: 713,
		kanji: '政',
		strokes: 9,
		meanings: &["politics"],
		on_yomis: &["sei"],
		kun_yomis: &["matsurigoto"]
	},
	Kanji {
		id: 714,
		kanji: '故',
		strokes: 9,
		meanings: &["circumstances"],
		on_yomis: &["ko"],
		kun_yomis: &["yue"]
	},
	Kanji {
		id: 715,
		kanji: '敵',
		strokes: 15,
		meanings: &["enemy"],
		on_yomis: &["teki"],
		kun_yomis: &["kataki"]
	},
	Kanji {
		id: 716,
		kanji: '断',
		strokes: 11,
		meanings: &["cut off"],
		on_yomis: &["dan"],
		kun_yomis: &["ta-tsu","kotowa-ru"]
	},
	Kanji {
		id: 717,
		kanji: '旧',
		strokes: 5,
		meanings: &["old times"],
		on_yomis: &["kyū"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 718,
		kanji: '易',
		strokes: 8,
		meanings: &["easy"],
		on_yomis: &["eki"],
		kun_yomis: &["yasa-shii"]
	},
	Kanji {
		id: 719,
		kanji: '暴',
		strokes: 15,
		meanings: &["outburst"],
		on_yomis: &["bō"],
		kun_yomis: &["aba-ku"]
	},
	Kanji {
		id: 720,
		kanji: '条',
		strokes: 7,
		meanings: &["clause"],
		on_yomis: &["jō"],
		kun_yomis: &["kudari"]
	},
	Kanji {
		id: 721,
		kanji: '枝',
		strokes: 8,
		meanings: &["branch"],
		on_yomis: &["shi"],
		kun_yomis: &["eda"]
	},
	Kanji {
		id: 722,
		kanji: '査',
		strokes: 9,
		meanings: &["investigate"],
		on_yomis: &["sa"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 723,
		kanji: '格',
		strokes: 10,
		meanings: &["status"],
		on_yomis: &["kaku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 724,
		kanji: '桜',
		strokes: 10,
		meanings: &["cherry"],
		on_yomis: &["ō"],
		kun_yomis: &["sakura"]
	},
	Kanji {
		id: 725,
		kanji: '検',
		strokes: 12,
		meanings: &["examine"],
		on_yomis: &["ken"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 726,
		kanji: '構',
		strokes: 14,
		meanings: &["construct"],
		on_yomis: &["kō"],
		kun_yomis: &["kama-eru"]
	},
	Kanji {
		id: 727,
		kanji: '武',
		strokes: 8,
		meanings: &["military"],
		on_yomis: &["bu","mu"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 728,
		kanji: '比',
		strokes: 4,
		meanings: &["compare"],
		on_yomis: &["hi"],
		kun_yomis: &["kura-beru"]
	},
	Kanji {
		id: 729,
		kanji: '永',
		strokes: 5,
		meanings: &["eternity"],
		on_yomis: &["ei"],
		kun_yomis: &["naga-i"]
	},
	Kanji {
		id: 730,
		kanji: '河',
		strokes: 8,
		meanings: &["stream"],
		on_yomis: &["ka"],
		kun_yomis: &["kawa"]
	},
	Kanji {
		id: 731,
		kanji: '液',
		strokes: 11,
		meanings: &["fluid"],
		on_yomis: &["eki"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 732,
		kanji: '混',
		strokes: 11,
		meanings: &["mix"],
		on_yomis: &["kon"],
		kun_yomis: &["ma-zeru"]
	},
	Kanji {
		id: 733,
		kanji: '減',
		strokes: 12,
		meanings: &["decrease"],
		on_yomis: &["gen"],
		kun_yomis: &["he-ru"]
	},
	Kanji {
		id: 734,
		kanji: '測',
		strokes: 12,
		meanings: &["measure"],
		on_yomis: &["soku"],
		kun_yomis: &["haka-ru"]
	},
	Kanji {
		id: 735,
		kanji: '準',
		strokes: 13,
		meanings: &["standard"],
		on_yomis: &["jun"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 736,
		kanji: '演',
		strokes: 14,
		meanings: &["perform"],
		on_yomis: &["en"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 737,
		kanji: '潔',
		strokes: 15,
		meanings: &["undefiled"],
		on_yomis: &["ketsu"],
		kun_yomis: &["isagiyo-i"]
	},
	Kanji {
		id: 738,
		kanji: '災',
		strokes: 7,
		meanings: &["disaster"],
		on_yomis: &["sai"],
		kun_yomis: &["wazawa-i"]
	},
	Kanji {
		id: 739,
		kanji: '燃',
		strokes: 16,
		meanings: &["burn"],
		on_yomis: &["nen"],
		kun_yomis: &["mo-eru"]
	},
	Kanji {
		id: 740,
		kanji: '版',
		strokes: 8,
		meanings: &["printing block"],
		on_yomis: &["han"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 741,
		kanji: '犯',
		strokes: 5,
		meanings: &["crime"],
		on_yomis: &["han"],
		kun_yomis: &["oka-su"]
	},
	Kanji {
		id: 742,
		kanji: '状',
		strokes: 7,
		meanings: &["form"],
		on_yomis: &["jō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 743,
		kanji: '独',
		strokes: 9,
		meanings: &["alone"],
		on_yomis: &["doku"],
		kun_yomis: &["hito-ri"]
	},
	Kanji {
		id: 744,
		kanji: '率',
		strokes: 11,
		meanings: &["rate"],
		on_yomis: &["ritsu","sotsu"],
		kun_yomis: &["hiki-iru"]
	},
	Kanji {
		id: 745,
		kanji: '現',
		strokes: 11,
		meanings: &["appear"],
		on_yomis: &["gen"],
		kun_yomis: &["arawa-reru"]
	},
	Kanji {
		id: 746,
		kanji: '留',
		strokes: 10,
		meanings: &["detain"],
		on_yomis: &["ryū","ru"],
		kun_yomis: &["todo-maru"]
	},
	Kanji {
		id: 747,
		kanji: '略',
		strokes: 11,
		meanings: &["abbreviation"],
		on_yomis: &["ryaku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 748,
		kanji: '益',
		strokes: 10,
		meanings: &["benefit"],
		on_yomis: &["eki"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 749,
		kanji: '眼',
		strokes: 11,
		meanings: &["eyeball"],
		on_yomis: &["gan"],
		kun_yomis: &["me"]
	},
	Kanji {
		id: 750,
		kanji: '破',
		strokes: 10,
		meanings: &["rend"],
		on_yomis: &["ha"],
		kun_yomis: &["yabu-ru"]
	},
	Kanji {
		id: 751,
		kanji: '確',
		strokes: 15,
		meanings: &["certain"],
		on_yomis: &["kaku"],
		kun_yomis: &["tashi-ka"]
	},
	Kanji {
		id: 752,
		kanji: '示',
		strokes: 5,
		meanings: &["indicate"],
		on_yomis: &["shi"],
		kun_yomis: &["shime-su"]
	},
	Kanji {
		id: 753,
		kanji: '祖',
		strokes: 9,
		meanings: &["ancestor"],
		on_yomis: &["so"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 754,
		kanji: '禁',
		strokes: 13,
		meanings: &["prohibition"],
		on_yomis: &["kin"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 755,
		kanji: '移',
		strokes: 11,
		meanings: &["shift"],
		on_yomis: &["i"],
		kun_yomis: &["utsu-ru"]
	},
	Kanji {
		id: 756,
		kanji: '程',
		strokes: 12,
		meanings: &["extent"],
		on_yomis: &["tei"],
		kun_yomis: &["hodo"]
	},
	Kanji {
		id: 757,
		kanji: '税',
		strokes: 12,
		meanings: &["tax"],
		on_yomis: &["zei"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 758,
		kanji: '築',
		strokes: 16,
		meanings: &["fabricate"],
		on_yomis: &["chiku"],
		kun_yomis: &["kizu-ku"]
	},
	Kanji {
		id: 759,
		kanji: '精',
		strokes: 14,
		meanings: &["refined"],
		on_yomis: &["sei"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 760,
		kanji: '素',
		strokes: 10,
		meanings: &["elementary"],
		on_yomis: &["su","so"],
		kun_yomis: &["moto"]
	},
	Kanji {
		id: 761,
		kanji: '経',
		strokes: 11,
		meanings: &["manage"],
		on_yomis: &["kei","kyō"],
		kun_yomis: &["he-ru"]
	},
	Kanji {
		id: 762,
		kanji: '統',
		strokes: 12,
		meanings: &["relationship"],
		on_yomis: &["tō"],
		kun_yomis: &["su-beru"]
	},
	Kanji {
		id: 763,
		kanji: '絶',
		strokes: 12,
		meanings: &["discontinue"],
		on_yomis: &["zetsu"],
		kun_yomis: &["ta-tsu"]
	},
	Kanji {
		id: 764,
		kanji: '綿',
		strokes: 14,
		meanings: &["cotton"],
		on_yomis: &["men"],
		kun_yomis: &["wata"]
	},
	Kanji {
		id: 765,
		kanji: '総',
		strokes: 14,
		meanings: &["whole"],
		on_yomis: &["sō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 766,
		kanji: '編',
		strokes: 15,
		meanings: &["compile"],
		on_yomis: &["hen"],
		kun_yomis: &["a-mu"]
	},
	Kanji {
		id: 767,
		kanji: '績',
		strokes: 17,
		meanings: &["exploits"],
		on_yomis: &["seki"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 768,
		kanji: '織',
		strokes: 18,
		meanings: &["weave"],
		on_yomis: &["shiki"],
		kun_yomis: &["o-ru"]
	},
	Kanji {
		id: 769,
		kanji: '罪',
		strokes: 13,
		meanings: &["guilt"],
		on_yomis: &["zai"],
		kun_yomis: &["tsumi"]
	},
	Kanji {
		id: 770,
		kanji: '群',
		strokes: 13,
		meanings: &["flock"],
		on_yomis: &["gun"],
		kun_yomis: &["mu-reru"]
	},
	Kanji {
		id: 771,
		kanji: '義',
		strokes: 13,
		meanings: &["righteousness"],
		on_yomis: &["gi"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 772,
		kanji: '耕',
		strokes: 10,
		meanings: &["till"],
		on_yomis: &["kō"],
		kun_yomis: &["tagaya-su"]
	},
	Kanji {
		id: 773,
		kanji: '職',
		strokes: 18,
		meanings: &["employment"],
		on_yomis: &["shoku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 774,
		kanji: '肥',
		strokes: 8,
		meanings: &["fertilizer"],
		on_yomis: &["hi"],
		kun_yomis: &["ko-yasu"]
	},
	Kanji {
		id: 775,
		kanji: '能',
		strokes: 10,
		meanings: &["ability"],
		on_yomis: &["nō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 776,
		kanji: '興',
		strokes: 16,
		meanings: &["entertain"],
		on_yomis: &["kyō"],
		kun_yomis: &["oko-su"]
	},
	Kanji {
		id: 777,
		kanji: '舌',
		strokes: 6,
		meanings: &["tongue"],
		on_yomis: &["zetsu"],
		kun_yomis: &["shita"]
	},
	Kanji {
		id: 778,
		kanji: '舎',
		strokes: 8,
		meanings: &["cottage"],
		on_yomis: &["sha"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 779,
		kanji: '術',
		strokes: 11,
		meanings: &["art"],
		on_yomis: &["jutsu"],
		kun_yomis: &["sube"]
	},
	Kanji {
		id: 780,
		kanji: '衛',
		strokes: 16,
		meanings: &["defense"],
		on_yomis: &["ei"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 781,
		kanji: '製',
		strokes: 14,
		meanings: &["manufacture"],
		on_yomis: &["sei"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 782,
		kanji: '複',
		strokes: 14,
		meanings: &["duplicate"],
		on_yomis: &["fuku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 783,
		kanji: '規',
		strokes: 11,
		meanings: &["rule"],
		on_yomis: &["ki"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 784,
		kanji: '解',
		strokes: 13,
		meanings: &["untie"],
		on_yomis: &["ge","kai"],
		kun_yomis: &["to-ku"]
	},
	Kanji {
		id: 785,
		kanji: '設',
		strokes: 11,
		meanings: &["establish"],
		on_yomis: &["setsu"],
		kun_yomis: &["mou-keru"]
	},
	Kanji {
		id: 786,
		kanji: '許',
		strokes: 11,
		meanings: &["permit"],
		on_yomis: &["kyo"],
		kun_yomis: &["yuru-su"]
	},
	Kanji {
		id: 787,
		kanji: '証',
		strokes: 12,
		meanings: &["evidence"],
		on_yomis: &["shō"],
		kun_yomis: &["akashi"]
	},
	Kanji {
		id: 788,
		kanji: '評',
		strokes: 12,
		meanings: &["evaluate"],
		on_yomis: &["hyō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 789,
		kanji: '講',
		strokes: 17,
		meanings: &["lecture"],
		on_yomis: &["kō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 790,
		kanji: '謝',
		strokes: 17,
		meanings: &["apologize"],
		on_yomis: &["sha"],
		kun_yomis: &["ayama-ru"]
	},
	Kanji {
		id: 791,
		kanji: '識',
		strokes: 19,
		meanings: &["discriminating"],
		on_yomis: &["shiki"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 792,
		kanji: '護',
		strokes: 20,
		meanings: &["safeguard"],
		on_yomis: &["go"],
		kun_yomis: &["mamo-ru"]
	},
	Kanji {
		id: 793,
		kanji: '豊',
		strokes: 13,
		meanings: &["bountiful"],
		on_yomis: &["hō"],
		kun_yomis: &["yuta-ka"]
	},
	Kanji {
		id: 794,
		kanji: '財',
		strokes: 10,
		meanings: &["wealth"],
		on_yomis: &["zai"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 795,
		kanji: '貧',
		strokes: 11,
		meanings: &["poor"],
		on_yomis: &["hin"],
		kun_yomis: &["mazu-shii"]
	},
	Kanji {
		id: 796,
		kanji: '責',
		strokes: 11,
		meanings: &["blame"],
		on_yomis: &["seki"],
		kun_yomis: &["se-meru"]
	},
	Kanji {
		id: 797,
		kanji: '貸',
		strokes: 12,
		meanings: &["lend"],
		on_yomis: &["tai"],
		kun_yomis: &["ka-su"]
	},
	Kanji {
		id: 798,
		kanji: '貿',
		strokes: 12,
		meanings: &["trade"],
		on_yomis: &["bō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 799,
		kanji: '賀',
		strokes: 12,
		meanings: &["congratulations"],
		on_yomis: &["ga"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 800,
		kanji: '資',
		strokes: 13,
		meanings: &["resources"],
		on_yomis: &["shi"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 801,
		kanji: '賛',
		strokes: 15,
		meanings: &["approve"],
		on_yomis: &["san"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 802,
		kanji: '質',
		strokes: 15,
		meanings: &["quality"],
		on_yomis: &["shitsu"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 803,
		kanji: '輸',
		strokes: 16,
		meanings: &["transport"],
		on_yomis: &["yu"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 804,
		kanji: '述',
		strokes: 8,
		meanings: &["mention"],
		on_yomis: &["jutsu"],
		kun_yomis: &["no-beru"]
	},
	Kanji {
		id: 805,
		kanji: '迷',
		strokes: 9,
		meanings: &["astray"],
		on_yomis: &["mei"],
		kun_yomis: &["mayo-u"]
	},
	Kanji {
		id: 806,
		kanji: '退',
		strokes: 9,
		meanings: &["retreat"],
		on_yomis: &["tai"],
		kun_yomis: &["shirizo-ku"]
	},
	Kanji {
		id: 807,
		kanji: '逆',
		strokes: 9,
		meanings: &["inverted"],
		on_yomis: &["gyaku"],
		kun_yomis: &["saka-rau"]
	},
	Kanji {
		id: 808,
		kanji: '造',
		strokes: 10,
		meanings: &["create"],
		on_yomis: &["zō"],
		kun_yomis: &["tsuku-ru"]
	},
	Kanji {
		id: 809,
		kanji: '過',
		strokes: 12,
		meanings: &["pass","exceed"],
		on_yomis: &["ka"],
		kun_yomis: &["sugi-ru"]
	},
	Kanji {
		id: 810,
		kanji: '適',
		strokes: 14,
		meanings: &["suitable"],
		on_yomis: &["teki"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 811,
		kanji: '酸',
		strokes: 14,
		meanings: &["acid"],
		on_yomis: &["san"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 812,
		kanji: '鉱',
		strokes: 13,
		meanings: &["mineral"],
		on_yomis: &["kō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 813,
		kanji: '銅',
		strokes: 14,
		meanings: &["copper"],
		on_yomis: &["dō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 814,
		kanji: '銭',
		strokes: 14,
		meanings: &["coin"],
		on_yomis: &["sen"],
		kun_yomis: &["zeni"]
	},
	Kanji {
		id: 815,
		kanji: '防',
		strokes: 7,
		meanings: &["resist"],
		on_yomis: &["bō"],
		kun_yomis: &["fuse-gu"]
	},
	Kanji {
		id: 816,
		kanji: '限',
		strokes: 9,
		meanings: &["limit"],
		on_yomis: &["gen"],
		kun_yomis: &["kagi-ru"]
	},
	Kanji {
		id: 817,
		kanji: '険',
		strokes: 11,
		meanings: &["precipitous"],
		on_yomis: &["ken"],
		kun_yomis: &["kewa-shii"]
	},
	Kanji {
		id: 818,
		kanji: '際',
		strokes: 14,
		meanings: &["occasion"],
		on_yomis: &["sai"],
		kun_yomis: &["kiwa"]
	},
	Kanji {
		id: 819,
		kanji: '雑',
		strokes: 14,
		meanings: &["miscellaneous"],
		on_yomis: &["zatsu"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 820,
		kanji: '非',
		strokes: 8,
		meanings: &["negative"],
		on_yomis: &["hi"],
		kun_yomis: &["ara-zu"]
	},
	Kanji {
		id: 821,
		kanji: '預',
		strokes: 13,
		meanings: &["deposit"],
		on_yomis: &["yo"],
		kun_yomis: &["azu-keru"]
	},
	Kanji {
		id: 822,
		kanji: '領',
		strokes: 14,
		meanings: &["territory"],
		on_yomis: &["ryō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 823,
		kanji: '額',
		strokes: 18,
		meanings: &["amount"],
		on_yomis: &["gaku"],
		kun_yomis: &["hitai"]
	},
	Kanji {
		id: 824,
		kanji: '飼',
		strokes: 13,
		meanings: &["domesticate"],
		on_yomis: &["shi"],
		kun_yomis: &["ka-u"]
	},
	Kanji {
		id: 825,
		kanji: '並',
		strokes: 8,
		meanings: &["row"],
		on_yomis: &["hei"],
		kun_yomis: &["nami","nara-bu"]
	},
	Kanji {
		id: 826,
		kanji: '乱',
		strokes: 7,
		meanings: &["chaos"],
		on_yomis: &["ran"],
		kun_yomis: &["mida-reru"]
	},
	Kanji {
		id: 827,
		kanji: '乳',
		strokes: 8,
		meanings: &["milk"],
		on_yomis: &["nyū"],
		kun_yomis: &["chichi"]
	},
	Kanji {
		id: 828,
		kanji: '亡',
		strokes: 3,
		meanings: &["deceased"],
		on_yomis: &["bō"],
		kun_yomis: &["na-kunaru"]
	},
	Kanji {
		id: 829,
		kanji: '仁',
		strokes: 4,
		meanings: &["kindness"],
		on_yomis: &["jin"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 830,
		kanji: '供',
		strokes: 8,
		meanings: &["offer"],
		on_yomis: &["kyō","ku"],
		kun_yomis: &["tomo"]
	},
	Kanji {
		id: 831,
		kanji: '俳',
		strokes: 10,
		meanings: &["actor"],
		on_yomis: &["hai"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 832,
		kanji: '値',
		strokes: 10,
		meanings: &["value"],
		on_yomis: &["chi"],
		kun_yomis: &["atai"]
	},
	Kanji {
		id: 833,
		kanji: '傷',
		strokes: 13,
		meanings: &["wound"],
		on_yomis: &["shō"],
		kun_yomis: &["kizu"]
	},
	Kanji {
		id: 834,
		kanji: '優',
		strokes: 17,
		meanings: &["superior"],
		on_yomis: &["yū"],
		kun_yomis: &["yasa-shii"]
	},
	Kanji {
		id: 835,
		kanji: '党',
		strokes: 10,
		meanings: &["political party"],
		on_yomis: &["tō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 836,
		kanji: '冊',
		strokes: 5,
		meanings: &["counter for books"],
		on_yomis: &["satsu"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 837,
		kanji: '処',
		strokes: 5,
		meanings: &["dispose"],
		on_yomis: &["sho"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 838,
		kanji: '刻',
		strokes: 8,
		meanings: &["engrave"],
		on_yomis: &["koku"],
		kun_yomis: &["kiza-mu"]
	},
	Kanji {
		id: 839,
		kanji: '割',
		strokes: 12,
		meanings: &["divide"],
		on_yomis: &["katsu"],
		kun_yomis: &["wa-ru"]
	},
	Kanji {
		id: 840,
		kanji: '創',
		strokes: 12,
		meanings: &["create"],
		on_yomis: &["sō"],
		kun_yomis: &["tsuku-ru"]
	},
	Kanji {
		id: 841,
		kanji: '劇',
		strokes: 15,
		meanings: &["drama"],
		on_yomis: &["geki"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 842,
		kanji: '勤',
		strokes: 12,
		meanings: &["diligence"],
		on_yomis: &["kin"],
		kun_yomis: &["tsuto-meru"]
	},
	Kanji {
		id: 843,
		kanji: '危',
		strokes: 6,
		meanings: &["dangerous"],
		on_yomis: &["ki"],
		kun_yomis: &["abu-nai"]
	},
	Kanji {
		id: 844,
		kanji: '卵',
		strokes: 7,
		meanings: &["egg"],
		on_yomis: &["ran"],
		kun_yomis: &["tamago"]
	},
	Kanji {
		id: 845,
		kanji: '厳',
		strokes: 17,
		meanings: &["strict"],
		on_yomis: &["gen"],
		kun_yomis: &["kibi-shii"]
	},
	Kanji {
		id: 846,
		kanji: '収',
		strokes: 4,
		meanings: &["obtain"],
		on_yomis: &["shū"],
		kun_yomis: &["osa-meru"]
	},
	Kanji {
		id: 847,
		kanji: '后',
		strokes: 6,
		meanings: &["queen"],
		on_yomis: &["kō","gō"],
		kun_yomis: &["kisaki"]
	},
	Kanji {
		id: 848,
		kanji: '否',
		strokes: 7,
		meanings: &["negate"],
		on_yomis: &["hi"],
		kun_yomis: &["ina","iya"]
	},
	Kanji {
		id: 849,
		kanji: '吸',
		strokes: 6,
		meanings: &["suck"],
		on_yomis: &["kyū"],
		kun_yomis: &["su-u"]
	},
	Kanji {
		id: 850,
		kanji: '呼',
		strokes: 8,
		meanings: &["call"],
		on_yomis: &["ko"],
		kun_yomis: &["yo-bu"]
	},
	Kanji {
		id: 851,
		kanji: '善',
		strokes: 12,
		meanings: &["virtue"],
		on_yomis: &["zen"],
		kun_yomis: &["yo-i"]
	},
	Kanji {
		id: 852,
		kanji: '困',
		strokes: 7,
		meanings: &["quandary"],
		on_yomis: &["kon"],
		kun_yomis: &["koma-ru"]
	},
	Kanji {
		id: 853,
		kanji: '垂',
		strokes: 8,
		meanings: &["droop"],
		on_yomis: &["sui"],
		kun_yomis: &["ta-reru"]
	},
	Kanji {
		id: 854,
		kanji: '城',
		strokes: 9,
		meanings: &["castle"],
		on_yomis: &["jō"],
		kun_yomis: &["shiro"]
	},
	Kanji {
		id: 855,
		kanji: '域',
		strokes: 11,
		meanings: &["range"],
		on_yomis: &["iki"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 856,
		kanji: '奏',
		strokes: 9,
		meanings: &["play music"],
		on_yomis: &["sō"],
		kun_yomis: &["kana-deru"]
	},
	Kanji {
		id: 857,
		kanji: '奮',
		strokes: 16,
		meanings: &["stirred up"],
		on_yomis: &["fun"],
		kun_yomis: &["furu-u"]
	},
	Kanji {
		id: 858,
		kanji: '姿',
		strokes: 9,
		meanings: &["shape"],
		on_yomis: &["shi"],
		kun_yomis: &["sugata"]
	},
	Kanji {
		id: 859,
		kanji: '存',
		strokes: 6,
		meanings: &["suppose"],
		on_yomis: &["son"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 860,
		kanji: '孝',
		strokes: 7,
		meanings: &["filial piety"],
		on_yomis: &["kō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 861,
		kanji: '宅',
		strokes: 6,
		meanings: &["home"],
		on_yomis: &["taku"],
		kun_yomis: &["ie"]
	},
	Kanji {
		id: 862,
		kanji: '宇',
		strokes: 6,
		meanings: &["eaves"],
		on_yomis: &["u"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 863,
		kanji: '宗',
		strokes: 8,
		meanings: &["religion"],
		on_yomis: &["shū"],
		kun_yomis: &["sō"]
	},
	Kanji {
		id: 864,
		kanji: '宙',
		strokes: 8,
		meanings: &["mid-air"],
		on_yomis: &["chū"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 865,
		kanji: '宝',
		strokes: 8,
		meanings: &["treasure"],
		on_yomis: &["hō"],
		kun_yomis: &["takara"]
	},
	Kanji {
		id: 866,
		kanji: '宣',
		strokes: 9,
		meanings: &["proclaim"],
		on_yomis: &["sen"],
		kun_yomis: &["notama-u"]
	},
	Kanji {
		id: 867,
		kanji: '密',
		strokes: 11,
		meanings: &["secrecy"],
		on_yomis: &["mitsu"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 868,
		kanji: '寸',
		strokes: 3,
		meanings: &["measurement"],
		on_yomis: &["sun"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 869,
		kanji: '専',
		strokes: 9,
		meanings: &["specialty"],
		on_yomis: &["sen"],
		kun_yomis: &["moppa-ra"]
	},
	Kanji {
		id: 870,
		kanji: '射',
		strokes: 10,
		meanings: &["shoot"],
		on_yomis: &["sha"],
		kun_yomis: &["i-ru"]
	},
	Kanji {
		id: 871,
		kanji: '将',
		strokes: 10,
		meanings: &["leader"],
		on_yomis: &["shō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 872,
		kanji: '尊',
		strokes: 12,
		meanings: &["revered"],
		on_yomis: &["son"],
		kun_yomis: &["tōto-bu"]
	},
	Kanji {
		id: 873,
		kanji: '就',
		strokes: 12,
		meanings: &["concerning"],
		on_yomis: &["shū"],
		kun_yomis: &["tsu-ku"]
	},
	Kanji {
		id: 874,
		kanji: '尺',
		strokes: 4,
		meanings: &["measure of length"],
		on_yomis: &["shaku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 875,
		kanji: '届',
		strokes: 8,
		meanings: &["deliver"],
		on_yomis: &["kai"],
		kun_yomis: &["todo-ku"]
	},
	Kanji {
		id: 876,
		kanji: '展',
		strokes: 10,
		meanings: &["expand"],
		on_yomis: &["ten"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 877,
		kanji: '層',
		strokes: 14,
		meanings: &["stratum"],
		on_yomis: &["sō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 878,
		kanji: '己',
		strokes: 3,
		meanings: &["self"],
		on_yomis: &["ko"],
		kun_yomis: &["onore"]
	},
	Kanji {
		id: 879,
		kanji: '巻',
		strokes: 9,
		meanings: &["scroll"],
		on_yomis: &["kan"],
		kun_yomis: &["maki"]
	},
	Kanji {
		id: 880,
		kanji: '幕',
		strokes: 13,
		meanings: &["curtain"],
		on_yomis: &["maku","baku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 881,
		kanji: '干',
		strokes: 3,
		meanings: &["dry"],
		on_yomis: &["kan"],
		kun_yomis: &["ho-su"]
	},
	Kanji {
		id: 882,
		kanji: '幼',
		strokes: 5,
		meanings: &["infancy"],
		on_yomis: &["yō"],
		kun_yomis: &["osana-i"]
	},
	Kanji {
		id: 883,
		kanji: '庁',
		strokes: 5,
		meanings: &["government office"],
		on_yomis: &["chō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 884,
		kanji: '座',
		strokes: 10,
		meanings: &["sit"],
		on_yomis: &["za"],
		kun_yomis: &["suwa-ru"]
	},
	Kanji {
		id: 885,
		kanji: '延',
		strokes: 8,
		meanings: &["prolong"],
		on_yomis: &["en"],
		kun_yomis: &["no-basu"]
	},
	Kanji {
		id: 886,
		kanji: '律',
		strokes: 9,
		meanings: &["rhythm"],
		on_yomis: &["ritsu"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 887,
		kanji: '従',
		strokes: 10,
		meanings: &["obey"],
		on_yomis: &["jū"],
		kun_yomis: &["shitaga-u"]
	},
	Kanji {
		id: 888,
		kanji: '忘',
		strokes: 7,
		meanings: &["forget"],
		on_yomis: &["bō"],
		kun_yomis: &["wasu-reru"]
	},
	Kanji {
		id: 889,
		kanji: '忠',
		strokes: 8,
		meanings: &["loyalty"],
		on_yomis: &["chū"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 890,
		kanji: '憲',
		strokes: 16,
		meanings: &["constitution"],
		on_yomis: &["ken"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 891,
		kanji: '我',
		strokes: 7,
		meanings: &["ego"],
		on_yomis: &["ga"],
		kun_yomis: &["ware"]
	},
	Kanji {
		id: 892,
		kanji: '批',
		strokes: 7,
		meanings: &["criticism"],
		on_yomis: &["hi"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 893,
		kanji: '担',
		strokes: 8,
		meanings: &["shouldering"],
		on_yomis: &["tan"],
		kun_yomis: &["nina-u"]
	},
	Kanji {
		id: 894,
		kanji: '拝',
		strokes: 8,
		meanings: &["worship"],
		on_yomis: &["hai"],
		kun_yomis: &["oga-mu"]
	},
	Kanji {
		id: 895,
		kanji: '拡',
		strokes: 8,
		meanings: &["broaden"],
		on_yomis: &["kaku"],
		kun_yomis: &["hiro-geru"]
	},
	Kanji {
		id: 896,
		kanji: '捨',
		strokes: 11,
		meanings: &["discard"],
		on_yomis: &["sha"],
		kun_yomis: &["su-teru"]
	},
	Kanji {
		id: 897,
		kanji: '探',
		strokes: 11,
		meanings: &["look for","search"],
		on_yomis: &["tan"],
		kun_yomis: &["saga-su"]
	},
	Kanji {
		id: 898,
		kanji: '推',
		strokes: 11,
		meanings: &["infer"],
		on_yomis: &["sui"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 899,
		kanji: '揮',
		strokes: 12,
		meanings: &["brandish"],
		on_yomis: &["ki"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 900,
		kanji: '操',
		strokes: 16,
		meanings: &["maneuver"],
		on_yomis: &["sō"],
		kun_yomis: &["ayatsu-ru"]
	},
	Kanji {
		id: 901,
		kanji: '敬',
		strokes: 12,
		meanings: &["respect"],
		on_yomis: &["kei"],
		kun_yomis: &["uyama-u"]
	},
	Kanji {
		id: 902,
		kanji: '映',
		strokes: 9,
		meanings: &["reflect"],
		on_yomis: &["ei"],
		kun_yomis: &["utsu-ru"]
	},
	Kanji {
		id: 903,
		kanji: '晩',
		strokes: 12,
		meanings: &["nightfall"],
		on_yomis: &["ban"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 904,
		kanji: '暖',
		strokes: 13,
		meanings: &["warmth"],
		on_yomis: &["dan"],
		kun_yomis: &["atata-kai"]
	},
	Kanji {
		id: 905,
		kanji: '暮',
		strokes: 14,
		meanings: &["livelihood"],
		on_yomis: &["bo"],
		kun_yomis: &["ku-rasu"]
	},
	Kanji {
		id: 906,
		kanji: '朗',
		strokes: 10,
		meanings: &["melodious"],
		on_yomis: &["rō"],
		kun_yomis: &["hoga-raka"]
	},
	Kanji {
		id: 907,
		kanji: '机',
		strokes: 6,
		meanings: &["desk"],
		on_yomis: &["ki"],
		kun_yomis: &["tsukue"]
	},
	Kanji {
		id: 908,
		kanji: '枚',
		strokes: 8,
		meanings: &["sheet of..."],
		on_yomis: &["mai"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 909,
		kanji: '染',
		strokes: 9,
		meanings: &["dye"],
		on_yomis: &["sen"],
		kun_yomis: &["so-meru"]
	},
	Kanji {
		id: 910,
		kanji: '株',
		strokes: 10,
		meanings: &["stocks"],
		on_yomis: &["shu"],
		kun_yomis: &["kabu"]
	},
	Kanji {
		id: 911,
		kanji: '棒',
		strokes: 12,
		meanings: &["rod"],
		on_yomis: &["bō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 912,
		kanji: '模',
		strokes: 14,
		meanings: &["imitation"],
		on_yomis: &["mo","bo"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 913,
		kanji: '権',
		strokes: 15,
		meanings: &["rights"],
		on_yomis: &["ken"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 914,
		kanji: '樹',
		strokes: 16,
		meanings: &["trees"],
		on_yomis: &["ju"],
		kun_yomis: &["ki"]
	},
	Kanji {
		id: 915,
		kanji: '欲',
		strokes: 11,
		meanings: &["longing"],
		on_yomis: &["yoku"],
		kun_yomis: &["ho-shii"]
	},
	Kanji {
		id: 916,
		kanji: '段',
		strokes: 9,
		meanings: &["steps"],
		on_yomis: &["dan"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 917,
		kanji: '沿',
		strokes: 8,
		meanings: &["run alongside"],
		on_yomis: &["en"],
		kun_yomis: &["so-u"]
	},
	Kanji {
		id: 918,
		kanji: '泉',
		strokes: 9,
		meanings: &["fountain"],
		on_yomis: &["sen"],
		kun_yomis: &["izumi"]
	},
	Kanji {
		id: 919,
		kanji: '洗',
		strokes: 9,
		meanings: &["wash"],
		on_yomis: &["sen"],
		kun_yomis: &["ara-u"]
	},
	Kanji {
		id: 920,
		kanji: '派',
		strokes: 9,
		meanings: &["sect"],
		on_yomis: &["ha"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 921,
		kanji: '済',
		strokes: 11,
		meanings: &["settle"],
		on_yomis: &["sai"],
		kun_yomis: &["su-mu"]
	},
	Kanji {
		id: 922,
		kanji: '源',
		strokes: 13,
		meanings: &["source"],
		on_yomis: &["gen"],
		kun_yomis: &["minamoto"]
	},
	Kanji {
		id: 923,
		kanji: '潮',
		strokes: 15,
		meanings: &["tide"],
		on_yomis: &["chō"],
		kun_yomis: &["shio"]
	},
	Kanji {
		id: 924,
		kanji: '激',
		strokes: 16,
		meanings: &["violent"],
		on_yomis: &["geki"],
		kun_yomis: &["hage-shii"]
	},
	Kanji {
		id: 925,
		kanji: '灰',
		strokes: 6,
		meanings: &["ashes"],
		on_yomis: &["kai"],
		kun_yomis: &["hai"]
	},
	Kanji {
		id: 926,
		kanji: '熟',
		strokes: 15,
		meanings: &["ripen"],
		on_yomis: &["juku"],
		kun_yomis: &["u-reru"]
	},
	Kanji {
		id: 927,
		kanji: '片',
		strokes: 4,
		meanings: &["one-sided"],
		on_yomis: &["hen"],
		kun_yomis: &["kata"]
	},
	Kanji {
		id: 928,
		kanji: '班',
		strokes: 10,
		meanings: &["corps"],
		on_yomis: &["han"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 929,
		kanji: '異',
		strokes: 11,
		meanings: &["uncommon"],
		on_yomis: &["i"],
		kun_yomis: &["koto-naru"]
	},
	Kanji {
		id: 930,
		kanji: '疑',
		strokes: 14,
		meanings: &["doubt"],
		on_yomis: &["gi"],
		kun_yomis: &["utaga-u"]
	},
	Kanji {
		id: 931,
		kanji: '痛',
		strokes: 12,
		meanings: &["pain"],
		on_yomis: &["tsū"],
		kun_yomis: &["ita-i"]
	},
	Kanji {
		id: 932,
		kanji: '皇',
		strokes: 9,
		meanings: &["emperor"],
		on_yomis: &["kō","ō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 933,
		kanji: '盛',
		strokes: 11,
		meanings: &["prosper"],
		on_yomis: &["sei"],
		kun_yomis: &["mo-ru"]
	},
	Kanji {
		id: 934,
		kanji: '盟',
		strokes: 13,
		meanings: &["alliance"],
		on_yomis: &["mei"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 935,
		kanji: '看',
		strokes: 9,
		meanings: &["watch over"],
		on_yomis: &["kan"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 936,
		kanji: '砂',
		strokes: 9,
		meanings: &["sand"],
		on_yomis: &["sa","sha"],
		kun_yomis: &["suna"]
	},
	Kanji {
		id: 937,
		kanji: '磁',
		strokes: 14,
		meanings: &["magnet"],
		on_yomis: &["ji"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 938,
		kanji: '私',
		strokes: 7,
		meanings: &["me"],
		on_yomis: &["shi"],
		kun_yomis: &["watakushi","watashi"]
	},
	Kanji {
		id: 939,
		kanji: '秘',
		strokes: 10,
		meanings: &["secret"],
		on_yomis: &["hi"],
		kun_yomis: &["hi-meru"]
	},
	Kanji {
		id: 940,
		kanji: '穀',
		strokes: 14,
		meanings: &["cereal"],
		on_yomis: &["koku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 941,
		kanji: '穴',
		strokes: 5,
		meanings: &["hole"],
		on_yomis: &["ketsu"],
		kun_yomis: &["ana"]
	},
	Kanji {
		id: 942,
		kanji: '窓',
		strokes: 11,
		meanings: &["window"],
		on_yomis: &["sō"],
		kun_yomis: &["mado"]
	},
	Kanji {
		id: 943,
		kanji: '筋',
		strokes: 12,
		meanings: &["muscle"],
		on_yomis: &["kin"],
		kun_yomis: &["suji"]
	},
	Kanji {
		id: 944,
		kanji: '策',
		strokes: 12,
		meanings: &["scheme"],
		on_yomis: &["saku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 945,
		kanji: '簡',
		strokes: 18,
		meanings: &["simplicity"],
		on_yomis: &["kan"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 946,
		kanji: '糖',
		strokes: 16,
		meanings: &["sugar"],
		on_yomis: &["tō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 947,
		kanji: '系',
		strokes: 7,
		meanings: &["lineage"],
		on_yomis: &["kei"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 948,
		kanji: '紅',
		strokes: 9,
		meanings: &["crimson"],
		on_yomis: &["kō"],
		kun_yomis: &["beni","kurenai"]
	},
	Kanji {
		id: 949,
		kanji: '納',
		strokes: 10,
		meanings: &["settlement"],
		on_yomis: &["nō"],
		kun_yomis: &["osa-meru"]
	},
	Kanji {
		id: 950,
		kanji: '純',
		strokes: 10,
		meanings: &["genuine"],
		on_yomis: &["jun"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 951,
		kanji: '絹',
		strokes: 13,
		meanings: &["silk"],
		on_yomis: &["ken"],
		kun_yomis: &["kinu"]
	},
	Kanji {
		id: 952,
		kanji: '縦',
		strokes: 16,
		meanings: &["vertical"],
		on_yomis: &["jū"],
		kun_yomis: &["tate"]
	},
	Kanji {
		id: 953,
		kanji: '縮',
		strokes: 17,
		meanings: &["shrink"],
		on_yomis: &["shuku"],
		kun_yomis: &["chidi-mu"]
	},
	Kanji {
		id: 954,
		kanji: '署',
		strokes: 13,
		meanings: &["government office"],
		on_yomis: &["sho"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 955,
		kanji: '翌',
		strokes: 11,
		meanings: &["forthcoming"],
		on_yomis: &["yoku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 956,
		kanji: '聖',
		strokes: 13,
		meanings: &["holy"],
		on_yomis: &["sei"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 957,
		kanji: '肺',
		strokes: 9,
		meanings: &["lung"],
		on_yomis: &["hai"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 958,
		kanji: '背',
		strokes: 9,
		meanings: &["back"],
		on_yomis: &["hai"],
		kun_yomis: &["se"]
	},
	Kanji {
		id: 959,
		kanji: '胸',
		strokes: 10,
		meanings: &["chest","breast"],
		on_yomis: &["kyō"],
		kun_yomis: &["mune"]
	},
	Kanji {
		id: 960,
		kanji: '脳',
		strokes: 11,
		meanings: &["brain"],
		on_yomis: &["nō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 961,
		kanji: '腹',
		strokes: 13,
		meanings: &["abdomen"],
		on_yomis: &["fuku"],
		kun_yomis: &["hara"]
	},
	Kanji {
		id: 962,
		kanji: '臓',
		strokes: 19,
		meanings: &["entrails"],
		on_yomis: &["zō"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 963,
		kanji: '臨',
		strokes: 18,
		meanings: &["lookover"],
		on_yomis: &["rin"],
		kun_yomis: &["nozo-mu"]
	},
	Kanji {
		id: 964,
		kanji: '至',
		strokes: 6,
		meanings: &["climax"],
		on_yomis: &["shi"],
		kun_yomis: &["ita-ru"]
	},
	Kanji {
		id: 965,
		kanji: '若',
		strokes: 8,
		meanings: &["young"],
		on_yomis: &["jaku"],
		kun_yomis: &["waka-i"]
	},
	Kanji {
		id: 966,
		kanji: '著',
		strokes: 11,
		meanings: &["renowned"],
		on_yomis: &["cho"],
		kun_yomis: &["arawa-su","ichijiru-shii"]
	},
	Kanji {
		id: 967,
		kanji: '蒸',
		strokes: 13,
		meanings: &["foment"],
		on_yomis: &["jō"],
		kun_yomis: &["mu-su"]
	},
	Kanji {
		id: 968,
		kanji: '蔵',
		strokes: 15,
		meanings: &["storehouse"],
		on_yomis: &["zō"],
		kun_yomis: &["kura"]
	},
	Kanji {
		id: 969,
		kanji: '蚕',
		strokes: 10,
		meanings: &["silkworm"],
		on_yomis: &["san"],
		kun_yomis: &["kaiko"]
	},
	Kanji {
		id: 970,
		kanji: '衆',
		strokes: 12,
		meanings: &["masses"],
		on_yomis: &["shū"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 971,
		kanji: '裁',
		strokes: 12,
		meanings: &["judge"],
		on_yomis: &["sai"],
		kun_yomis: &["saba-ku"]
	},
	Kanji {
		id: 972,
		kanji: '装',
		strokes: 12,
		meanings: &["attire"],
		on_yomis: &["sō","shō"],
		kun_yomis: &["yosoo-u"]
	},
	Kanji {
		id: 973,
		kanji: '裏',
		strokes: 13,
		meanings: &["rear"],
		on_yomis: &["ri"],
		kun_yomis: &["ura"]
	},
	Kanji {
		id: 974,
		kanji: '補',
		strokes: 12,
		meanings: &["supplement"],
		on_yomis: &["ho"],
		kun_yomis: &["ogina-u"]
	},
	Kanji {
		id: 975,
		kanji: '視',
		strokes: 11,
		meanings: &["look at"],
		on_yomis: &["shi"],
		kun_yomis: &["mi-ru"]
	},
	Kanji {
		id: 976,
		kanji: '覧',
		strokes: 17,
		meanings: &["perusal"],
		on_yomis: &["ran"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 977,
		kanji: '討',
		strokes: 10,
		meanings: &["chastise"],
		on_yomis: &["tō"],
		kun_yomis: &["u-tsu"]
	},
	Kanji {
		id: 978,
		kanji: '訪',
		strokes: 11,
		meanings: &["visit"],
		on_yomis: &["hō"],
		kun_yomis: &["tazu-neru"]
	},
	Kanji {
		id: 979,
		kanji: '訳',
		strokes: 11,
		meanings: &["translate","reason"],
		on_yomis: &["yaku"],
		kun_yomis: &["wake"]
	},
	Kanji {
		id: 980,
		kanji: '詞',
		strokes: 12,
		meanings: &["term"],
		on_yomis: &["shi"],
		kun_yomis: &["kotoba"]
	},
	Kanji {
		id: 981,
		kanji: '誌',
		strokes: 14,
		meanings: &["document"],
		on_yomis: &["shi"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 982,
		kanji: '認',
		strokes: 14,
		meanings: &["recognize"],
		on_yomis: &["nin"],
		kun_yomis: &["mito-meru"]
	},
	Kanji {
		id: 983,
		kanji: '誕',
		strokes: 15,
		meanings: &["born"],
		on_yomis: &["tan"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 984,
		kanji: '誠',
		strokes: 13,
		meanings: &["sincerity"],
		on_yomis: &["sei"],
		kun_yomis: &["makoto"]
	},
	Kanji {
		id: 985,
		kanji: '誤',
		strokes: 14,
		meanings: &["mistake"],
		on_yomis: &["go"],
		kun_yomis: &["ayama-ru"]
	},
	Kanji {
		id: 986,
		kanji: '論',
		strokes: 15,
		meanings: &["argument","discussion"],
		on_yomis: &["ron"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 987,
		kanji: '諸',
		strokes: 15,
		meanings: &["various"],
		on_yomis: &["sho"],
		kun_yomis: &["moro"]
	},
	Kanji {
		id: 988,
		kanji: '警',
		strokes: 19,
		meanings: &["admonish"],
		on_yomis: &["kei"],
		kun_yomis: &["imashi-meru"]
	},
	Kanji {
		id: 989,
		kanji: '貴',
		strokes: 12,
		meanings: &["precious"],
		on_yomis: &["ki"],
		kun_yomis: &["tatto-i"]
	},
	Kanji {
		id: 990,
		kanji: '賃',
		strokes: 13,
		meanings: &["fare"],
		on_yomis: &["chin"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 991,
		kanji: '遺',
		strokes: 15,
		meanings: &["bequeath"],
		on_yomis: &["i"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 992,
		kanji: '郵',
		strokes: 11,
		meanings: &["mail"],
		on_yomis: &["yū"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 993,
		kanji: '郷',
		strokes: 11,
		meanings: &["home town"],
		on_yomis: &["kyō"],
		kun_yomis: &["gō"]
	},
	Kanji {
		id: 994,
		kanji: '針',
		strokes: 10,
		meanings: &["needle"],
		on_yomis: &["shin"],
		kun_yomis: &["hari"]
	},
	Kanji {
		id: 995,
		kanji: '鋼',
		strokes: 16,
		meanings: &["steel"],
		on_yomis: &["kō"],
		kun_yomis: &["hagane"]
	},
	Kanji {
		id: 996,
		kanji: '閉',
		strokes: 11,
		meanings: &["closed"],
		on_yomis: &["hei"],
		kun_yomis: &["shi-meru"]
	},
	Kanji {
		id: 997,
		kanji: '閣',
		strokes: 14,
		meanings: &["tower"],
		on_yomis: &["kaku"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 998,
		kanji: '降',
		strokes: 10,
		meanings: &["descend"],
		on_yomis: &["kō"],
		kun_yomis: &["o-riru"]
	},
	Kanji {
		id: 999,
		kanji: '陛',
		strokes: 10,
		meanings: &["majesty"],
		on_yomis: &["hei"],
		kun_yomis: &[""]
	},
	Kanji {
		id: 1000,
		kanji: '除',
		strokes: 10,
		meanings: &["exclude"],
		on_yomis: &["jo","ji"],
		kun_yomis: &["nozo-ku"]
	},
	Kanji {
		id: 1001,
		kanji: '障',
		strokes: 14,
		meanings: &["hurt"],
		on_yomis: &["shō"],
		kun_yomis: &["sawa-ru"]
	},
	Kanji {
		id: 1002,
		kanji: '難',
		strokes: 18,
		meanings: &["difficult"],
		on_yomis: &["nan"],
		kun_yomis: &["muzuka-shii"]
	},
	Kanji {
		id: 1003,
		kanji: '革',
		strokes: 9,
		meanings: &["leather"],
		on_yomis: &["kaku"],
		kun_yomis: &["kawa"]
	},
	Kanji {
		id: 1004,
		kanji: '頂',
		strokes: 11,
		meanings: &["top","receive"],
		on_yomis: &["chō"],
		kun_yomis: &["itada-ku"]
	},
	Kanji {
		id: 1005,
		kanji: '骨',
		strokes: 10,
		meanings: &["bone"],
		on_yomis: &["kotsu"],
		kun_yomis: &["hone"]
	},
	Kanji {
		id: 1006,
		kanji: '茨',
		strokes: 9,
		meanings: &["caltrop"],
		on_yomis: &["shi","ji"],
		kun_yomis: &["ibara","kusabuki","kaya"]
	},
	Kanji {
		id: 1007,
		kanji: '媛',
		strokes: 12,
		meanings: &["beauty"],
		on_yomis: &["en"],
		kun_yomis: &["hime"]
	},
	Kanji {
		id: 1008,
		kanji: '岡',
		strokes: 8,
		meanings: &["hill"],
		on_yomis: &["kou"],
		kun_yomis: &["oka"]
	},
	Kanji {
		id: 1009,
		kanji: '潟',
		strokes: 15,
		meanings: &["lagoon"],
		on_yomis: &["seki"],
		kun_yomis: &["kata"]
	},
	Kanji {
		id: 1010,
		kanji: '岐',
		strokes: 7,
		meanings: &["high"],
		on_yomis: &["ki","gi"],
		kun_yomis: &["edamichi"]
	},
	Kanji {
		id: 1011,
		kanji: '熊',
		strokes: 14,
		meanings: &["bear"],
		on_yomis: &["yuu"],
		kun_yomis: &["kuma"]
	},
	Kanji {
		id: 1012,
		kanji: '香',
		strokes: 9,
		meanings: &["fragrant"],
		on_yomis: &["kou","kyou"],
		kun_yomis: &["kaori","ka"]
	},
	Kanji {
		id: 1013,
		kanji: '佐',
		strokes: 7,
		meanings: &["assist"],
		on_yomis: &["sa"],
		kun_yomis: &["tasuke","tasukeru","suke"]
	},
	Kanji {
		id: 1014,
		kanji: '埼',
		strokes: 11,
		meanings: &["headland"],
		on_yomis: &["ki"],
		kun_yomis: &["saki"]
	},
	Kanji {
		id: 1015,
		kanji: '崎',
		strokes: 11,
		meanings: &["rough"],
		on_yomis: &["ki"],
		kun_yomis: &["saki","kewashii"]
	},
	Kanji {
		id: 1016,
		kanji: '滋',
		strokes: 12,
		meanings: &["grow"],
		on_yomis: &["ju","shi"],
		kun_yomis: &["shigeru","masu","masumasu"]
	},
	Kanji {
		id: 1017,
		kanji: '鹿',
		strokes: 11,
		meanings: &["deer"],
		on_yomis: &["roku"],
		kun_yomis: &["shika","ka"]
	},
	Kanji {
		id: 1018,
		kanji: '縄',
		strokes: 15,
		meanings: &["rope"],
		on_yomis: &["jou","bin","you"],
		kun_yomis: &["nawa"]
	},
	Kanji {
		id: 1019,
		kanji: '井',
		strokes: 4,
		meanings: &["well"],
		on_yomis: &["sei","shou"],
		kun_yomis: &["i","ido"]
	},
	Kanji {
		id: 1020,
		kanji: '沖',
		strokes: 7,
		meanings: &["pour"],
		on_yomis: &["chuu"],
		kun_yomis: &["oki","waku"]
	},
	Kanji {
		id: 1021,
		kanji: '栃',
		strokes: 9,
		meanings: &["horse chestnut"],
		on_yomis: &[""],
		kun_yomis: &["tochi"]
	},
	Kanji {
		id: 1022,
		kanji: '奈',
		strokes: 8,
		meanings: &["but"],
		on_yomis: &["na","dai"],
		kun_yomis: &["karanashi"]
	},
	Kanji {
		id: 1023,
		kanji: '梨',
		strokes: 11,
		meanings: &["pear"],
		on_yomis: &["ri"],
		kun_yomis: &["nashi"]
	},
	Kanji {
		id: 1024,
		kanji: '阪',
		strokes: 6,
		meanings: &["heights/slope"],
		on_yomis: &["han"],
		kun_yomis: &["saka"]
	},
	Kanji {
		id: 1025,
		kanji: '阜',
		strokes: 8,
		meanings: &["mound"],
		on_yomis: &["fu"],
		kun_yomis: &["oka"]
	},
];
#[allow(dead_code)]
pub const FREQUENCY_RANK: [usize;1006] = [20, 0, 146, 220, 36, 21, 17, 9, 1, 57, 16, 89, 52, 2, 102, 118, 713, 246, 110, 207, 224, 46, 19, 123, 612, 141, 140, 556, 31, 622, 4, 365, 132, 291, 12, 422, 136, 395, 835, 145, 228, 152, 335, 142, 369, 133, 3, 285, 129, 219, 130, 144, 75, 276, 8, 51, 624, 53, 429, 44, 201, 48, 60, 278, 101, 249, 223, 286, 50, 629, 343, 40, 260, 153, 254, 41, 403, 523, 761, 165, 143, 534, 169, 555, 745, 188, 405, 112, 262, 29, 229, 7, 5, 587, 243, 435, 13, 116, 311, 559, 440, 206, 190, 699, 575, 605, 238, 657, 558, 300, 661, 100, 49, 18, 6, 522, 330, 232, 317, 149, 478, 267, 423, 274, 762, 444, 549, 298, 765, 472, 25, 184, 163, 168, 241, 268, 263, 822, 81, 370, 590, 419, 65, 284, 785, 648, 526, 80, 176, 430, 38, 466, 134, 216, 318, 913, 117, 362, 712, 384, 574, 588, 10, 920, 90, 182, 673, 921, 175, 513, 212, 150, 55, 462, 518, 784, 37, 92, 800, 245, 26, 270, 818, 722, 261, 434, 282, 486, 618, 58, 266, 191, 426, 139, 11, 185, 656, 982, 178, 158, 482, 215, 468, 221, 186, 544, 167, 451, 944, 433, 670, 643, 668, 467, 358, 457, 644, 233, 251, 557, 313, 339, 66, 84, 275, 532, 986, 95, 341, 504, 676, 252, 312, 573, 701, 315, 752, 497, 314, 39, 672, 938, 485, 281, 295, 96, 242, 120, 1003, 647, 303, 751, 62, 710, 421, 389, 525, 542, 137, 662, 733, 230, 88, 680, 519, 697, 736, 226, 250, 441, 406, 775, 567, 654, 447, 591, 371, 349, 723, 328, 930, 43, 809, 292, 82, 319, 757, 725, 63, 688, 61, 530, 971, 742, 181, 515, 173, 359, 666, 28, 773, 787, 27, 306, 211, 417, 830, 665, 304, 726, 23, 839, 172, 411, 616, 442, 225, 360, 610, 412, 192, 728, 1002, 815, 974, 74, 834, 498, 374, 846, 716, 239, 135, 70, 45, 350, 674, 376, 237, 783, 779, 792, 876, 702, 683, 652, 861, 505, 424, 470, 189, 975, 720, 689, 743, 988, 288, 380, 396, 510, 803, 978, 231, 409, 83, 372, 244, 213, 804, 552, 310, 97, 744, 364, 414, 296, 727, 171, 802, 520, 305, 609, 322, 613, 428, 855, 258, 474, 449, 780, 568, 22, 692, 902, 816, 103, 823, 639, 416, 277, 400, 771, 446, 458, 85, 533, 399, 893, 500, 806, 735, 617, 621, 808, 601, 910, 114, 705, 448, 227, 264, 694, 247, 265, 858, 272, 408, 997, 499, 755, 508, 970, 651, 166, 256, 788, 611, 537, 283, 965, 960, 547, 579, 393, 273, 378, 968, 626, 535, 541, 820, 607, 506, 320, 916, 337, 493, 79, 351, 59, 170, 122, 781, 578, 47, 361, 336, 794, 353, 791, 346, 850, 623, 598, 208, 994, 869, 898, 148, 131, 453, 483, 30, 432, 756, 564, 528, 583, 832, 235, 214, 463, 709, 490, 496, 93, 977, 410, 308, 234, 707, 87, 660, 180, 124, 580, 91, 248, 627, 837, 576, 717, 161, 890, 86, 338, 209, 294, 287, 154, 177, 924, 848, 487, 687, 539, 366, 947, 892, 104, 718, 456, 205, 67, 162, 418, 859, 32, 125, 636, 553, 280, 658, 77, 269, 479, 934, 884, 78, 750, 766, 71, 1000, 503, 998, 796, 825, 887, 15, 649, 631, 843, 708, 768, 64, 581, 895, 714, 437, 589, 293, 174, 691, 333, 203, 111, 397, 873, 210, 147, 255, 14, 929, 551, 620, 871, 127, 667, 845, 194, 521, 54, 682, 105, 991, 536, 798, 789, 24, 972, 987, 126, 760, 828, 841, 730, 597, 465, 912, 810, 679, 427, 681, 748, 115, 635, 461, 646, 754, 481, 807, 69, 415, 548, 640, 373, 344, 678, 719, 619, 866, 958, 324, 334, 571, 289, 401, 563, 394, 817, 606, 933, 222, 253, 671, 459, 669, 599, 786, 932, 963, 438, 954, 450, 746, 769, 455, 776, 630, 68, 922, 345, 840, 1001, 943, 511, 885, 198, 473, 492, 759, 659, 826, 529, 484, 514, 734, 793, 348, 632, 851, 663, 489, 491, 477, 747, 704, 633, 586, 321, 763, 388, 299, 159, 391, 413, 883, 480, 854, 386, 527, 877, 740, 592, 443, 94, 711, 327, 973, 867, 302, 517, 641, 767, 758, 614, 696, 732, 157, 891, 842, 402, 880, 684, 909, 352, 819, 706, 501, 852, 160, 833, 729, 966, 981, 301, 655, 653, 471, 550, 939, 279, 838, 512, 801, 329, 741, 686, 164, 307, 862, 138, 638, 603, 634, 179, 509, 915, 931, 953, 908, 685, 582, 782, 992, 540, 445, 543, 538, 645, 56, 331, 257, 897, 454, 584, 1005, 870, 875, 119, 420, 805, 677, 879, 899, 271, 739, 76, 996, 476, 99, 990, 524, 625, 33, 436, 989, 738, 905, 821, 569, 945, 193, 949, 914, 381, 962, 886, 797, 964, 863, 570, 864, 425, 814, 494, 770, 309, 379, 900, 183, 983, 790, 860, 642, 326, 197, 950, 637, 979, 849, 464, 799, 935, 259, 856, 955, 431, 698, 927, 993, 901, 918, 128, 695, 367, 565, 151, 878, 615, 340, 889, 452, 121, 202, 332, 917, 488, 377, 984, 888, 155, 385, 608, 562, 831, 382, 865, 959, 936, 495, 985, 690, 108, 956, 919, 778, 460, 363, 703, 113, 390, 872, 577, 375, 942, 531, 404, 323, 195, 715, 196, 731, 795, 604, 811, 107, 347, 753, 882, 398, 923, 545, 724, 204, 664, 995, 596, 561, 156, 952, 896, 199, 961, 827, 98, 948, 357, 240, 836, 475, 546, 35, 602, 42, 829, 675, 844, 354, 881, 1004, 34, 469, 355, 387, 572, 941, 904, 906, 812, 560, 957, 502, 824, 721, 926, 700, 554, 693, 407, 903, 999, 516, 325, 894, 109, 342, 911, 290, 297, 774, 946, 106, 594, 316, 650, 585, 356, 72, 764, 187, 813, 628, 976, 857, 749, 967, 772, 847, 507, 928, 737, 566, 200, 439, 980, 593, 868, 907, 937, 600, 925, 853, 940, 73, 236, 218, 595, 368, 777, 392, 951, 383, 874, 217, 969];