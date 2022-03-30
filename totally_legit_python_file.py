import time
import cv2
import pytesseract
import pyautogui

def print_ocr(s_width, s_hight, i_width, i_height, read_file, write_file):
    pyautogui.screenshot(read_file, region=(s_width,s_hight,i_height,i_width))
    img = cv2.imread('monkeytypesample.png')
    string = pytesseract.image_to_string(img).strip().replace("\n","")

    # Write to the txt file
    open(write_file, 'w').close()
    with open(write_file, 'w') as file:
        file.write(string)
    file.close()

# Reading the image
# print_ocr(500,500,1245,185, "monkeytypesample.png", 'initial-string.txt')
while True:    
    print_ocr(500,500,1245,185, "monkeytypesample.png", 'string.txt')

