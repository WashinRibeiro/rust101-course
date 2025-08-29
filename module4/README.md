# Condicionais

Os condicionais são estruturas que permitem ao seu programa tomar decisões baseadas em condições específicas.
Em Rust, if e else são utilizados para executar diferentes blocos de código dependendo do resultado de uma expressão booleana.
Operadores como >, <, >=, <=, ==, e != ajudam a formar essas expressões, permitindo comparações entre valores.

| Operação | Resultado |
| -------- | --------- |
| 1 > 0    | true      |
| 1 < 0    | false     |
| 1 >= 0   | true      |
| 1 <= 0   | false     |
| 1 == 0   | false     |
| 1 != 0   | true      |

# Tabela verdade

A tabela verdade é uma ferramenta fundamental para entender como operações lógicas como AND, OR, e NOT funcionam.
Em Rust, essas são representadas por &&, ||, e !, respectivamente. Essas operações permitem combinar ou inverter condições booleanas, que são cruciais para a lógica de decisão em programas.

| a     | b     | a AND b | a OR b | NOT a | NOT b |
| ----- | ----- | ------- | ------ | ----- | ----- |
| true  | true  | true    | true   | false | false |
| true  | false | false   | true   | false | true  |
| false | true  | false   | true   | true  | false |
| false | false | false   | false  | true  | true  |

# Repetições

Laços de repetição são estruturas fundamentais em qualquer linguagem de programação, permitindo que um bloco de código seja executado repetidamente sob certas condições.
Em Rust, os principais laços de repetição são while, o loop infinito loop e for.
O while executa um bloco de código enquanto uma condição especificada é verdadeira.
O loop cria um ciclo infinito, que só pode ser interrompido com uma declaração break.
O for, por sua vez, é usado para iterar sobre elementos de uma coleção ou valores em um intervalo, sendo uma forma concisa e poderosa de percorrer arrays, vetores, entre outros.