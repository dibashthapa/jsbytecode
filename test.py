import dis

class Matrix:
    def __init__(self, arr):
        self.arr = arr

    def __mul__(self, x):
        self.arr = [val* x for val in self.arr]
        return self.arr


marix  = Matrix([1,2,3])
second = Matrix([3,6,9])
print(marix*3*second*3)
