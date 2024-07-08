# 수명 규칙

1. 첫 번째 규칙은 컴파일러가 참조인 각 매개변수에 수명 매개변수를 할당한다는 것입니다. 즉, 매개변수가 하나인 함수는 수명 매개변수를 하나 얻습니다: fn foo<'a>(x: &'a i32); 매개변수가 두 개인 함수는 두 개의 별도 수명 매개변수를 얻습니다: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); 이런 식으로 계속됩니다.
2. 두 번째 규칙은 입력 수명 매개변수가 정확히 하나뿐인 경우 해당 수명이 모든 출력 수명 매개변수에 할당된다는 것입니다: fn foo<'a>(x: &'a i32) -> &'a i32.
3. 세 번째 규칙은 입력 수명 매개변수가 여러 개 있지만 그 중 하나가 &self 또는 &mut self인 경우(이것이 메서드이기 때문), self의 수명이 모든 출력 수명 매개변수에 할당된다는 것입니다. 이 세 번째 규칙은 필요한 심볼이 적기 때문에 메서드를 읽고 쓰기가 훨씬 더 편합니다.