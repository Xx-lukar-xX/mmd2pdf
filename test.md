
graph TD
	define["input(n,m,f)"]
	com1{{"n==m"}}
	com2{{"n>m"}}
	cal1["f=m+n"]
	cal2["f=n-m"]
	cal3["f=m-n"]
  start --> define
  define --> com1
  com1--true-->cal1
  com1--false-->com2
  com2--true-->cal2
  com2--false-->cal3
  cal1-->exit
  cal2-->exit
  cal3-->exit
