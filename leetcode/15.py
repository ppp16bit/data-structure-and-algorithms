class Solution:
  def threeSum(self, nums: list[int]) -> list[list[int]]:
    if len(nums) < 3:
      return []

    ans = []
    nums.sort()

    for i in range(len(nums) - 2):
      if nums[i] > 0:
        break

      if i > 0 and nums[i] == nums[i - 1]:
        continue
  
      l, r = i + 1, len(nums) - 1
      
      while l < r:
        summ = nums[i] + nums[l] + nums[r]
        if summ == 0:
          ans.append([nums[i], nums[l], nums[r]])
          
          l += 1
          while l < r and nums[l] == nums[l - 1]:
            l += 1
          
          r -= 1
          while l < r and nums[r] == nums[r + 1]:
            r -= 1

        elif summ < 0:
          l += 1
        else:
          r -= 1

    return ans