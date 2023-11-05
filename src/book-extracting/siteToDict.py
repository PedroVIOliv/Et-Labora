import pickle
import time
import requests
from lxml import html
import threading
from trecho import trecho


#create object to store in snippets that contains book, chapter, number and text

snippets = []

def get_trecho(url, xpath, number):
    
    while True:
        try:
            response = requests.get(url)
            break
        except:
            print("retrying " + url)
            continue

    # Parse the HTML content of the response
    tree = html.fromstring(response.content)

    # Use the XPath to extract the desired section of the HTML
    book = tree.xpath('/html/body/main/div[1]/a[2]/span')[0]
    chapter = tree.xpath('/html/body/main/div[1]/a[3]/span')[0]
    xnums = tree.xpath('count(/html/body/main/div[2]/article/div[1]/p)')
    toPrint = ""
    for x in range(1, int(xnums)+1):
        toPrint += tree.xpath('/html/body/main/div[2]/article/div[1]/p['+str(x)+']/text()')[0]
        toPrint += "\n"

    tags = tree.xpath('/html/body/main/div[2]/article/div[2]/div[2]/a')
    tagList = []
    for tag in tags:
        tagList.append(tag.text)
    snippets.append(trecho(book.text, chapter.text, number, toPrint, tagList))

# Define the URLs and XPath
urls = [f'https://escriva.org/pt-br/{book}/{number}/' for book, count in [('camino', 999), ('surco', 1000), ('forja', 1055)] for number in range(1, count+1)]
xpath = '/html/body/main/div[2]/article/div[1]'

# Create a thread for each URL
threads = []
for i, url in enumerate(urls):
    if(i%20 == 0):
        time.sleep(0.05)
    thread = threading.Thread(target=get_trecho, args=(url, xpath, i+1))
    threads.append(thread)
    thread.start()

# Wait for all threads to finish
for thread in threads:
    thread.join()
print(len(snippets))
#save snippets in pickle file
with open('snippets.pickle', 'wb') as handle:
    pickle.dump(snippets, handle, protocol=pickle.HIGHEST_PROTOCOL)


