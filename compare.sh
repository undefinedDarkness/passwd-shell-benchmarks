for i in getshells getshells-go getshells.py getshells.pl getshells.awk getshells.ps1
do
  echo "################################################"
  echo $i
  time ./${i}
done

