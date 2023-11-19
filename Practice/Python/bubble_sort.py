def bubble_sort(arr):
    for i in range(len(arr)-1):
        for j in range(len(arr)-i-1):
            if arr[j] > arr[j+1]:
                arr[j], arr[j+1] = arr[j+1], arr[j]

if __name__ == "__main__":
    arr = [77, 2, 17, 8, 1, 12, 5, 3, 7, 11, 25]
    print('before sort :', arr)
    bubble_sort(arr)
    print('after sort :', arr)
    # before sort : [77, 2, 17, 8, 1, 12, 5, 3, 7, 11, 25]
    # after sort : [1, 2, 3, 5, 7, 8, 11, 12, 17, 25, 77]
