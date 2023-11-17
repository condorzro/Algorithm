def selection_sort(arr):
    for i in range(len(arr)-1):
        min = i+1
        for j in range(i+1, len(arr)):
            if arr[min] > arr[j]:
                min = j
        if arr[i] > arr[min]:
            arr[i], arr[min] = arr[min], arr[i]

if __name__ == '__main__':
    arr = [2, 17, 8, 12, 5, 3, 7, 11]
    print('before sort ->', arr)
    selection_sort(arr)
    print('after aort ->', arr)
    # output
    # before sort -> [2, 17, 8, 12, 5, 3, 7, 11]
    # after aort -> [2, 3, 5, 7, 8, 11, 12, 17]
