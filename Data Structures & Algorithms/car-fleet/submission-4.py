class Solution:
    def carFleet(self, target: int, position: List[int], speed: List[int]) -> int:
        # 1. 將位置和速度組合成一對，並依照位置從「大到小」排序（從離終點最近的開始）
        cars = sorted(zip(position, speed), reverse=True)
        
        # 2. 計算每輛車在不受阻礙下，到達終點所需的時間
        # 時間 = (目標距離 - 當前位置) / 速度
        times = [(target - p) / s for p, s in cars]
        
        fleets = 0
        current_max_time = 0 # 記錄當前車隊到達終點的最晚時間
        
        # 3. 遍歷每輛車的時間
        for t in times:
            # 如果當前這輛車的時間比前一個車隊還要久
            # 代表它追不上前方的車隊，會自己帶頭形成一個「新車隊」
            if t > current_max_time:
                fleets += 1
                current_max_time = t
                
        return fleets