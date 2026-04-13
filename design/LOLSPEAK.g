grammar LOLSPEAK;

lolspeak:   	HAI body KBYE;
body:    	body_type body|;
body_type:	comment|paragraph|bold|italics|list|new_line|link|variable_definition|variable|plain_text; 

comment:	OBTW plain_text TLDR;

head:		MAEK HEAD title MKAY;
title:	        GIMMEH TITLE plain_text OIC;

paragraph:		MAEK PARAGRAF paragraph_context MKAY;
paragraph_context:	paragraph_type paragraph_context|;
paragraph_type:		bold|italics|list|item|new_line|link|variable_definition|variable|plain_text;

bold: 		GIMMEH BOLD plain_text OIC;

italics:	GIMMEH ITALICS plain_text OIC ;

list:		MAEK LIST an_list MKAY;
an_list:	item an_list|item;
item:		GIMMEH ITEM item_content OIC;

new_line:	NEWLINE;

link:		GIMMEH LINX address OIC;

variable_definition :	IHAZ variable_name ITIZ plain_text MKAY;
variable            :	LEMMESEE variable_name OIC;


 
 
 
//Lexer Rules
HAI       : '#HAI' ;
KBYE      : '#KBYE' ;
OBTW      : '#OBTW' ;
TLDR      : '#TLDR' ;
MAEK      : '#MAEK' ;
HEAD      : 'HEAD' ;
GIMMEH    : '#GIMMEH' ;
TITLE     : 'TITLE' ;
OIC       : '#OIC' ;
PARAGRAF  : 'PARAGRAF' ;
BOLD      : 'BOLD' ;
ITALICS   : 'ITALICS' ;
LIST      : 'LIST' ;
ITEM      : 'ITEM' ;
NEWLINE   : '#NEWLINE' ;
LINX      : 'LINX' ;
IHAZ      : '#IHAZ' ;
ITIZ      : '#ITIZ' ;
LEMMESEE  : '#LEMMESEE' ;
MKAY      : '#MKAY' ;

//Undefined helper rules

plain_text    : WORD;
item_content  :	WORD;
address	      :	WORD;
variable_name :	WORD;  

WORD	:	('A'..'Z'|'a'..'z'|'0'..'9')+; 

