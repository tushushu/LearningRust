from dataclasses import dataclass
from typing import Optional
from random import random


def main() -> None:
    # # 0 and 1. Native Python does not support declaring pointers,
    # # and user cannot deciding passing by value or reference.
    # # 2. Using Optional[int] type
    # x: Optional[int] = 100
    # print(x)

    # x = None
    # print(x)

    # # 3. Using Optional[Student] type
    # @dataclass
    # class Student:
    #     age: int

    # def random_student() -> Optional[Student]:
    #     if random() > 0.5:
    #         return None
    #     return Student(age=18)

    # s = random_student()
    # print(s.age)

    # 4. Overloading assignment
    class Student:
        def __init__(self, v: Optional[int]) -> None:
            self.__age = v

        @property
        def age(self) -> Optional[int]:
            return self.__age

        @age.setter
        def age(self, v: Optional[int]) -> None:
            if v is None:
                raise ValueError("Age should not be None!")
            self.__age = v

    s = Student(None)
    s = Student(18)
    s.age = None


if __name__ == '__main__':
    main()
