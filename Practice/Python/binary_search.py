def binary_search(arr, target):
    if len(arr) < 1:
        return False
    else:
        mid_idx = len(arr) // 2
        if arr[mid_idx] == target:
            return True
        elif len(arr) == 1:
            return False
        elif arr[mid_idx] > target:
            return binary_search(arr[:mid_idx], target)
        elif arr[mid_idx] < target:
            return binary_search(arr[mid_idx+1:], target)
    return False

if __name__ == '__main__':
    arr = [77, 2, 17, 8, 1, 12, 5, 3, 7, 11, 25]
    arr.sort()
    target = 76
    print(arr)
    print('find', target)
    print('Result :', binary_search(arr, target))
