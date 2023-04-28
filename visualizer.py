# -*- coding: utf-8 -*-
"""
Created on Sat Jan  7 22:24:59 2023

@author: Rebecca
"""

import sys
import yaml
import numpy as np
import matplotlib.pyplot as plt
from matplotlib.offsetbox import AnchoredText


#%% Plotsettings

text_fontsize = 32
title_fontsize = 43
tick_fontsize = 28
plt.rcParams.update({
    "text.usetex": True,       # change if you want Tex labels
    "font.family": "sans-serif",
    'font.sans-serif': 'Helvetica'
})


#%% manually set parameters

# !!!!!!!!!!!!!!!!!!!!!!! HERE !!!!!!!!!!!!!!!!!!!!!!!!!!! (start)

save = True
# save = False

#ITR = True
ITR = False

#study = 'study1'
#study = 'study2'
#study = 'study3'
#study = 'study3_full'
#study = 'study4'
study = 'study5'
#study = 'study6'
#study = 'study7'
#study = 'study8'
#study = 'paper/1'
# study = 'paper/2'


version = 'SCC'
#version = 'S2V'

# Learn Config
learn_seq= "base_configs/learn_sequences/study1.txt"
#learn_seq= "base_configs/learn_sequences/study7.txt"

# !!!!!!!!!!!!!!!!!!!!!! HERE !!!!!!!!!!!!!!!!!!!!!!!!!!!!! (end)

#%% commandline arguments

sim_config = str("./variants/{study}/sim/".format(study = study)) + str(sys.argv[1]) + ".yaml"

with open(sim_config, 'r') as file:
    sim_config_var = yaml.safe_load(file)

#study = 'study5_2'
#%%  Parameters

# Simulation Times

# max simulation time in s (f64):
t_sim= sim_config_var['t_sim']
# symbol duration (spray duration + pause duration[min. 0.03]) in s (f64):
t_sym= sim_config_var['t_sym']
# spray duration in s (f64):
t_sp= sim_config_var['t_sp']
# delta time step (either 1/camera fps or something smaller to use sim2vidtool) ins s (f64):
t_dts= sim_config_var['t_dts']


# Layout

# distance between both inj im m (f64):
d= sim_config_var['d']
# distance offset in m from d/2 (f64):
dy= sim_config_var['dy']


# Receiver Props

# cam_fps:
cam_fps= sim_config_var['cam_fps']
# z Position (Sprayer z is 0) in m (f64):
cam_z= sim_config_var['cam_z']
# height - how far can the camera see - commonly tube diameter (camera looks in negative z direction) in m (f64): 
cam_height= sim_config_var['cam_height']
# width on tube floor - maximum width seen in landscape picture im m (f64)
cam_w_proj= sim_config_var['cam_w_proj']
# camera ratio length (f64)
cam_ratio_l= sim_config_var['cam_ratio_l']      #maybe name that y
# camera ratio width (f64)
cam_ratio_w= sim_config_var['cam_ratio_w']      #maybe name that x


# Sender Props

# molecules_per_spray (mps) (usize):
mps= sim_config_var['mps']
# initial velocity / velocity mean in m/s (f64):
v_mean= sim_config_var['v_mean']
# velocity sigma (f64):
v_sigma= sim_config_var['v_sigma']
# distribution_sigma for x and z angle (f64):
dist_sigma= sim_config_var['dist_sigma']
# liter_molecule_conversion (None if you want to use mps) in mol/liter (f64):
#lmc= null

# Offset for Inj far in symbol duration (t_sym) if you only got measurements in seconds just divide by t_sym (f64):
off_f= sim_config_var['off_f']
# Message repetitions for Inj far (u32):
rep_f= sim_config_var['rep_f']
# Message path for Inj far (&str):
#msg_f: Msg_f
msg_f= sim_config_var['msg_f']
#msg_f_out: Msg_f_out
# Offset for Inj near in symbol duration (t_sym) (f64):
off_n= sim_config_var['off_n']
# Message repetitions for Inj near (u32):
rep_n= sim_config_var['rep_n']
# Message path for Inj near (&str):
msg_n= sim_config_var['msg_n']
#msg_n_out: Msg_n_out




#%%

# Output file

if version == 'SCC' or (version == 'S2V' and study == 'study7'):
    liv_path= sim_config_var['liv_path']
else:
    liv_path = "./sim2vid/livs/" + str(sys.argv[1]) + ".txt"


# Apply Config
#frame = 

# apply_config = sys.argv[2]

# with open(apply_config, 'r') as file:
#     apply_config_var = yaml.safe_load(file)

# frame = apply_config_var['frame']

if version == 'S2V':
    t_dts = 1/cam_fps


fps = 1/t_dts



d_travel_near = (d/2) - dy
travel_mean_near = ((d/2.0)-dy) / v_mean
d_travel_far = (d/2) + dy
travel_mean_far = d_travel_far / v_mean

#frames per symbol
frpsym = int(t_sym//t_dts)  
 
frame = int((travel_mean_near + t_sp/3)*fps)

if study == 'study3_full':
    if d < 1.4:
        frame = 32
    elif d < 1.8 and t_sym < 0.06:
        frame = 40
    elif d < 1.8:
        frame = 46
    elif d < 2.2:
        frame = 52
    elif d < 2.6:
        frame = 58
    elif d < 3.0 and t_sym < 0.08:
        frame = 63
    elif d < 3.0:
        frame = 68
    else:
        frame = 78
        
if study == 'study4':
    if d < 1.6:
        frame = 34
    elif d < 1.8 and t_sym < 0.06:
        frame = 43
    elif d < 1.8:
        if off_f < 0.2:
            frame = 46
        else:
            frame = 42
    elif d < 2.2:
        frame = 52
    elif d < 2.6 and off_f > 0:
        frame = 58
    elif d < 2.6:
        frame = 62
    elif d < 3.0 and off_f > 0:
        frame = 68
    elif d < 3.0:
        frame = 72
    elif d < 3.4 and off_n > 0.46:
        frame = 81
    else:
        frame = 78
        
if study == 'study5':
    if d < 1.4:
        fr_n = int((travel_mean_near + 5*t_sp/12)*fps)
        fr_f = int((travel_mean_far + 5*t_sp/12)*fps)
    elif d < 2.0:
        fr_n = int((travel_mean_near + t_sp/3)*fps)
        fr_f = int((travel_mean_far + t_sp/3)*fps)
    elif d < 3.0:
        fr_n = int((travel_mean_near + t_sp/4)*fps)
        fr_f = int((travel_mean_far + t_sp/4)*fps)

        
if study == 'study5' or study == 'study6' or study == 'study7' or study == 'paper/1' or study == 'paper/2':
    apply_config = str("./variants/{study}/apply_config/".format(study = study)) + str(sys.argv[1]) + "_0.yaml"
    
    if study == 'study7' or study == 'paper/1' or study == 'paper/2':
        apply_config = str("./variants/{study}/apply_config/".format(study = study) + str(sys.argv[2]) + ".yaml")
    
    with open(apply_config, 'r') as file:
        apply_config_var = yaml.safe_load(file)

    frame = apply_config_var['frame']
   
#%%

def codes_to_color(arr):
    arr_color = []    
    for code in arr:
        if code == 'h':
            arr_color.append('b')
        elif code == 'f':
            arr_color.append('r')
        elif code == 'n':
            arr_color.append('orange')
        elif code == 'l':
            arr_color.append('gray')
        else:
            arr_color.append('white')
    return arr_color
            
def codes_to_label(arr):
    #l = ["'11' - both", "'10' - far", "'01' - near", "'00' - none"]
    l = ["'11' - H", "'10' - F", "'01' - N", "'00' - L"]
    for i in range(4, len(arr)):
        l.append('')
    return l


def codes_to_alpha(arr, option):
    arr_alpha = np.array()
    if option == "n":
        for code in arr:
            if code == 'h':
                arr_alpha.append(0)
            elif code == 'f':
                arr_alpha.append(0.5)
            elif code == 'n':
                arr_alpha.append(1)
            elif code == 'l':
                arr_alpha.append(0)
            else:
                arr_alpha.append(0)
    elif option == "f":
        for code in arr:
            if code == 'h':
                arr_alpha.append(0)
            elif code == 'f':
                arr_alpha.append(1)
            elif code == 'n':
                arr_alpha.append(0.5)
            elif code == 'l':
                arr_alpha.append(0)
            else:
                arr_alpha.append(0)
        
    else:
        arr_alpha.append(1)
    
    return arr_alpha
        
  
#%%

def labels_fn(i):
    if i < 4:
        #l = ["'11' - both", "'10' - far", "'01' - near", "'00' - none"]
        l = ["'11' - H", "'10' - F", "'01' - N", "'00' - L"]
        return l[i]
    else:
        return ''
    

#%%

with open(learn_seq, 'r') as f:
    codes = f.readline().split(",")

codes = np.array(codes[:])
 
    
  
#%%


with open(liv_path, 'r') as f:
    vals = f.readline().split(";")
    
vals = np.array(vals[:-1], dtype= 'float64')

max_val = max(vals)

n_bit = (len(vals)-frame)//frpsym - 2
if study == 'study3':
    n_bit = (len(vals)-frame)//frpsym //2
    
if d < 1.4:
    off = int((d_travel_near - (cam_w_proj/2))/((v_mean + 0.3 * v_sigma) * t_dts))

elif d < 2.2:
    off = int((d_travel_near - (cam_w_proj/2))/((v_mean + 0.2 * v_sigma) * t_dts))
else:
    off = int((d_travel_near - (cam_w_proj/2))/((v_mean + 0.65 * v_sigma) * t_dts))

if study == 'study5_2':
    off = int((d_travel_near - (cam_w_proj/2))/((v_mean + 0 * v_sigma) * t_dts))

fig = plt.figure()
fig.set_figwidth(15)
fig.set_figheight(10)
plt.rc('xtick', labelsize=tick_fontsize)
plt.rc('ytick', labelsize=tick_fontsize)


# learn_seq
colors = codes_to_color(codes)
alphas = [1]
labels = [codes_to_label(codes)]
# by previous code
# colors = ['gray']
# for c in codes_to_color(codes)[:-1]:
#     colors.append(c)


# look at near
#alphas = codes_to_alpha(codes, "n")
# look at far
#alphas = codes_to_alpha(codes, "f")

if study == 'study3':
    x1 = np.arange(0+off, frpsym+off, 1)
    x2 = np.arange(frpsym+off-1, 2*frpsym+off, 1)
    for i in range(n_bit):
        y1 = np.divide(vals[(2*i*frpsym+off):((2*i+1)*frpsym+off)],max_val)
        y2 = np.divide(vals[((2*i+1)*frpsym+off-1):((2*i+2)*frpsym+off)], max_val)
        plt.plot(x1, y1, label = labels_fn(i), color= colors[(2*i)%len(colors)], alpha = alphas[i%len(alphas)])
        plt.plot(x2, y2, color= colors[(2*i)%len(colors)], alpha = alphas[i%len(alphas)])

# prints also some frames of the next bit for better ISI identification 
elif study =='study3_full' or study == 'study4' or study == 'study6' or study == 'study5' or (study == 'study7' or study == 'paper/1' or study == 'paper/2') and ITR == False:
    x1 = np.arange(0+off, frpsym+off, 1)
    x2 = np.arange(frpsym+off-1, frpsym+off-1+int(frpsym/3), 1)
    for i in range(n_bit):
        y1 = np.divide(vals[(i*frpsym+off):((i+1)*frpsym+off)],max_val)
        y2 = np.divide(vals[((i+1)*frpsym+off-1):((i+1)*frpsym+off-1)+int(frpsym/3)], max_val)
        plt.plot(x1, y1, label = labels_fn(i), color= colors[(i)%len(colors)], alpha = alphas[i%len(alphas)])
        plt.plot(x2, y2, color= colors[(i)%len(colors)], alpha = 0.35)

else:
    x = np.arange(0+off, frpsym+off, 1)
    
    for i in range(n_bit):
        y = np.divide(vals[(i*frpsym+off):((i+1)*frpsym+off)],max_val)
        plt.plot(x, y, label = labels_fn(i), color= colors[i%len(colors)], alpha = alphas[i%len(alphas)])



text_off = 0.2
    
plt.xlabel('Image Frames since Symbol Start', fontsize=text_fontsize)
plt.ylabel('LIV / Highest Detected LIV', fontsize=text_fontsize)
#plt.rc('font', family = 'sans-serif')



#plt.axvline(x=25, label="Max Diff 1st half", color= 'black', linestyle = 'dotted')
#plt.axvline(x=38, label="Max Diff 2nd half", color= 'black', linestyle = 'dashed')

# if study == 'study5':
#     plt.axvline(x=fr_n, label="Peak Near", color= 'black', linestyle = 'dashed')
#     plt.axvline(x=fr_f, label="Peak Far", color= 'black', linestyle = 'dotted')
if study == 'study5_2':
    frame = 36
    plt.axvline(x=frame, label="Reconstruction Frame", color= 'black', linestyle = 'dashed')
elif study == 'study1' or study == 'study2':
    True
else:
    plt.axvline(x=frame, label="Sampling Frame", color= 'black', linestyle = 'dashed', linewidth = 4)
#plt.axvline(x=40, label="Reconstruction Frame", color= 'black', linestyle = 'dashed')


if study == 'study3':
    r = 2
elif study == 'study3_full' or study == 'study4' or study == 'study5' or study == 'study6' or (study == 'study7' or study == 'paper/1' or study == 'paper/2') and ITR==False:
    r = 4/3
else:
    r = 1
    
if study == 'study1':
    k = 0.015
else:
    k = 0

plt.title('LIV Sorted by Case', fontsize=title_fontsize)
#plt.title('LIV by Prior Code - Near Tx', fontsize=title_fontsize)
#plt.title('LIV by Prior Code - Far Tx', fontsize=title_fontsize)

if (study == 'study7' or study == 'paper/1' or study == 'paper/2') and ITR == False:
    plt.text((r*frpsym+off)*1.2, 0.3-text_off, r'$d = {}\, m$'.format(d), fontsize=text_fontsize)
    plt.text((r*frpsym+off)*1.2, 0.2-text_off, r'$\Delta y = {}\, m$'.format(dy), fontsize=text_fontsize)
    plt.text((r*frpsym+off)*1.04, 0.35-text_off, r'$t_{sym} = {val}\, ms$'.format(sym = "{sym}", val = str(int(t_sym * 1000))), fontsize=text_fontsize)
    plt.text((r*frpsym+off)*1.04, 0.25-text_off, r'off\_$f = {}$'.format(round(off_f, 2)), fontsize=text_fontsize, style = 'italic')
    plt.text((r*frpsym+off)*1.04, 0.15-text_off, r'off\_$n = {}$'.format(round(off_n, 2)), fontsize=text_fontsize, style = 'italic')
else:
    if study == 'study2' or study == 'study3_full':
        plt.text((r*frpsym+off)*1.04, 0.5-text_off, r'$t_{spray} = {val}\, ms$'.format(spray = "{spray}", val = str(int(t_sp*1000))), fontsize=text_fontsize)
        plt.text((r*frpsym+off)*1.04, 0.4-text_off, r'$t_{pause} = {val}\, ms$'.format(pause = "{pause}", val = str(int(np.floor((t_sym - t_sp)*1000)))), fontsize=text_fontsize)
    elif study == 'study4' or study == 'study6':
        plt.text((r*frpsym+off)*1.04, 0.5-text_off, r'off\_$f = {}$'.format(round(off_f, 2)), fontsize=text_fontsize, style = 'italic')
        plt.text((r*frpsym+off)*1.04, 0.4-text_off, r'off\_$n = {}$'.format(round(off_n, 2)), fontsize=text_fontsize, style = 'italic')
    
    if study == 'study5':
        # plt.text((r*frpsym+off)*1.04, 0.5-text_off, r'$t_{sym} = {val}\, ms$'.format(sym = "{sym}", val = str(int(t_sym * 1000))), fontsize=text_fontsize)
        
        # just for the paper...
        plt.text((r*frpsym+off)*1.03, 0.375-text_off, r'$t_{sym} = {val}\, ms$'.format(sym = "{sym}", val = str(int(t_sym * 1000))), fontsize=text_fontsize)
        plt.text((r*frpsym+off)*1.03, 0.297-text_off, r'$\Delta t = 0.0$'.format(sym = "{sym}", val = str(int(t_sym * 1000))), fontsize=text_fontsize)
        
    else:
        plt.text((r*frpsym+off)*(1.04 - k), 0.6-text_off, r'$t_{sym} = {val}\, ms$'.format(sym = "{sym}", val = str(int(t_sym * 1000))), fontsize=text_fontsize)
    # plt.text((r*frpsym+off)*(1.04 - k), 0.3-text_off, r'$d = {}\, m$'.format(d), fontsize=text_fontsize)
    # plt.text((r*frpsym+off)*(1.04 - k), 0.2-text_off, r'$\Delta y = {}\, m$'.format(dy), fontsize=text_fontsize)

#just for paper
    plt.text((r*frpsym+off)*(1.03 - k), 0.218-text_off, r'$d = {}\, m$'.format(d), fontsize=text_fontsize)
    plt.text((r*frpsym+off)*(1.03 - k), 0.14-text_off, r'$\Delta y = {}\, m$'.format(dy), fontsize=text_fontsize)



if d < 2.1:
    tickdist = 2
elif d < 3.0:
    tickdist = 3
else:
    tickdist = 5
    
if (study == 'study7' or study == 'paper/1' or study == 'paper/2') and ITR == False  or study == "study5":
    plt.axhline(apply_config_var['th']['large']/max_val, label = "B Threshold", color = 'black', linestyle= (0,(1,1)), linewidth = 3)
    plt.axhline(apply_config_var['th']['medium']/max_val, label = "M Threshold", color = 'black', linestyle= (0,(1,3)), linewidth = 3)
    plt.axhline(apply_config_var['th']['small']/max_val, label = "S Threshold", color = 'black', linestyle= (0,(1,5)), linewidth = 3)
    # MOL-Eye
    
if study == 'study1' or study == 'study2':
    plt.xticks(np.arange(off, frpsym+off, tickdist))
else:
    plt.xticks(np.arange(off, frpsym+off+int(frpsym/3), tickdist))

#if study == 'study5' or study == 'study6':
plt.grid(color = 'gray', axis = 'x')

#plt.savefig("./study_results/study3/d_1.6/{dy}.png".format(dy = dy * 100), bbox_inches="tight")

plt.legend(bbox_to_anchor=(1,1), loc = "upper left", fontsize=text_fontsize)

if save:
    plt.savefig("./study_results/{study}/{version}/{name}.pdf".format(study = study, version = version, name = sys.argv[1]), bbox_inches="tight")

plt.show()

#%%

print(plt.rcParams['font.sans-serif'])


#%%

if study == 'study1' or study == 'study6':
    fig = plt.figure()
    fig.set_figwidth(2)
    fig.set_figheight(3)
    plt.rc('xtick', labelsize=13)
    plt.rc('ytick', labelsize=13)
    
    
    x= frame
    
    h = []
    f = []
    n = []
    l = []
    
    for i in range(n_bit):
        liv = np.divide(vals[(i*frpsym+frame)], max_val)
        
        group = codes[i%len(codes)]
        if group == 'h':
            h.append(liv)
        elif group == 'f':
            f.append(liv)
        elif group == 'n':
            n.append(liv)
        elif group == 'l':
            l.append(liv)
        else:
            raise Exception("code invalid")           
                
        plt.plot(x, liv, label = labels_fn(i), color= colors[i%len(colors)], marker = ".")
    
    h_mean = np.mean(h)
    f_mean = np.mean(f)
    n_mean = np.mean(n)
    l_mean = np.mean(l)
    
    plt.plot(x, h_mean, label = "H mean", color = "cyan", marker= "_")
    plt.plot(x, f_mean, label = "F mean", color = "firebrick", marker= "_")
    plt.plot(x, n_mean, label = "N mean", color = "darkgoldenrod", marker= "_")
    plt.plot(x, l_mean, label = "L mean", color = "black", marker= "_")
    
    if study == 'study1':
        print(sys.argv[1])
        print("Fraction f_mean/h_mean = " + str(f_mean/h_mean) + "\nFraction n_mean/h_mean = " + str(n_mean/h_mean))
    
    plt.legend(bbox_to_anchor=(1,1), loc = "upper left", fontsize=13)
    
    
    #plt.xlabel('Sim2Vid', fontsize=18)
    #plt.xlabel('SCC', fontsize=18)
    plt.xlabel('Reconstruction Frame')
    
    plt.ylabel('Relative LIV', fontsize=18)
    
    plt.title('LIVs in Reconstructionframe', fontsize=23)
    plt.xticks([x])
    
    
    if save:
        plt.savefig("./study_results/{study}/{version}/at_frame_{name}.pdf".format(study = study, version = version, name = sys.argv[1]), bbox_inches="tight")
    
    plt.show()

#%%

now = False

if study == 'study5' and now:
    fig = plt.figure()
    fig.set_figwidth(2)
    fig.set_figheight(3)
    plt.rc('xtick', labelsize=13)
    plt.rc('ytick', labelsize=13)
    
    
    N="Near Peak"
    F="Far Peak"
    
    n = []
    f = []
    
    for i in range(n_bit):
        group = codes[i%len(codes)]
        if group == 'f':
            liv = vals[(i*frpsym+fr_f)]
            f.append(liv)
            plt.plot(F, liv, color= 'r', marker = ".")
        elif group == 'n':
            liv = vals[(i*frpsym+fr_n)]
            n.append(liv)
            plt.plot(N, liv, color= 'orange', marker = ".")
                
    

    f_mean = np.mean(f)
    n_mean = np.mean(n)
    
    plt.plot(F, f_mean, label = "Far Peak Mean", color = "firebrick", marker= "_")
    plt.annotate("{}".format(f_mean), (13/12, 1/3-1/5), xycoords = 'axes fraction', color= "firebrick", fontsize = 13)
    plt.plot(N, n_mean, label = "Near Peak Mean", color = "darkgoldenrod", marker= "_")
    plt.annotate("{}".format(n_mean), (13/12, 2/3-1/5), xycoords = 'axes fraction', color = "darkgoldenrod", fontsize = 13)

    
    plt.legend(bbox_to_anchor=(1,1), loc = "upper left", fontsize=13)
    
    
    plt.xlabel('Frame', fontsize=18)
    
    plt.ylabel('LIV', fontsize=18)
    
    plt.title('Peak LIVs in Frames', fontsize=23)
    plt.xticks([N, F])
    
    print(sys.argv[1])
    print("N: " + str(n_mean))
    print("F: " + str(f_mean))
    
    
    if save:
        plt.savefig("./study_results/{study}/{version}/3/at_frame_{name}.pdf".format(study = study, version = version, name = sys.argv[1]), bbox_inches="tight")
    
    plt.show()

#%%

if study == 'study5_2':
    
    from scipy.stats import norm
    
    datapoints = 5000
    
    v = np.linspace(v_mean - 3*v_sigma, v_mean + 3*v_sigma, datapoints)

    nd = norm.pdf(v, loc = v_mean, scale = v_sigma)
    
    # with open('study_results/study5/Calc_diagrams/vel.txt', 'r') as f:
    #     v = f.readline().split(";")
    
    # v = v[:-1]


    # with open('study_results/study5/Calc_diagrams/normal_dist_rust.txt', 'r') as f:
    #     nd = f.readline().split(";")
    
    # nd = nd[:-1]
    
    fig = plt.figure()
    fig.set_figwidth(15)
    fig.set_figheight(10)
    plt.rc('xtick', labelsize=tick_fontsize)
    plt.rc('ytick', labelsize=tick_fontsize)
    
    plt.plot(v, nd, color = 'black', linewidth = 3)
    plt.axvline(x = v_mean, color= "c", label= 'Mean')
    plt.axvline(x = v_mean+v_sigma, color= 'gray', linestyle = 'dashed', label= r'$1\sigma$')
    plt.axvline(x = v_mean-v_sigma, color= 'gray', linestyle = 'dashed')
    plt.axvline(x = v_mean+3*v_sigma, color= 'gray', linestyle = 'dotted', label = r'$3\sigma$')
    plt.axvline(x = v_mean-3*v_sigma, color= 'gray', linestyle = 'dotted')
    plt.xlabel(r'Velocity $[\frac{{m}}{{s}}]$'.format(), fontsize=text_fontsize)
    plt.ylabel("Probability", fontsize=text_fontsize)
    plt.title("Probability-Distribution of Velocity in a Molecule", fontsize=title_fontsize)
    plt.legend(bbox_to_anchor=(1,1), loc = "upper left", fontsize = text_fontsize)
    
    #plt.savefig("./study_results/study5/Calc_diagrams/vel.pdf", bbox_inches="tight")
    
    plt.show()
    
#%%

    fig = plt.figure()
    fig.set_figwidth(15)
    fig.set_figheight(10)
    plt.rc('xtick', labelsize=tick_fontsize)
    plt.rc('ytick', labelsize=tick_fontsize)
    
    plt.plot(d_travel_far/v, nd, color = 'r', linewidth = 3)
    plt.axvline(x = d_travel_far/v_mean, color= "c", label= 'Mean')
    plt.axvline(x = d_travel_far/(v_mean+v_sigma), color= 'gray', linestyle = 'dashed', label= r'$1\sigma$')
    plt.axvline(x = d_travel_far/(v_mean-v_sigma), color= 'gray', linestyle = 'dashed')
    plt.axvline(x = d_travel_far/(v_mean+3*v_sigma), color= 'gray', linestyle = 'dotted', label = r'$3\sigma$')
    plt.axvline(x = d_travel_far/(v_mean-3*v_sigma), color= 'gray', linestyle = 'dotted')
    plt.xlabel(r'Travel Time $[s]$'.format(), fontsize=text_fontsize)
    plt.ylabel("Probability", fontsize=text_fontsize)
    plt.title("Distribution of Time of Arrival from Tx far", fontsize=title_fontsize)
    plt.legend(bbox_to_anchor=(1,1), loc = "upper left", fontsize = text_fontsize)
    plt.text(0.18, 0.035, r'$d = {}\, m$'.format(d), fontsize=text_fontsize)
    plt.text(0.18, 0.02, r'$\Delta y = {}\, m$'.format(dy), fontsize=text_fontsize)
    
    #plt.savefig("./study_results/study5/Calc_diagrams/time_f.pdf", bbox_inches="tight")
    
    plt.show()

    fig = plt.figure()
    fig.set_figwidth(15)
    fig.set_figheight(10)
    plt.rc('xtick', labelsize=tick_fontsize)
    plt.rc('ytick', labelsize=tick_fontsize)
    
    plt.plot(d_travel_near/v, nd, color = 'orange', linewidth = 3)
    plt.axvline(x = d_travel_near/v_mean, color= "c", label= 'Mean')
    plt.axvline(x = d_travel_near/(v_mean+v_sigma), color= 'gray', linestyle = 'dashed', label= r'$1\sigma$')
    plt.axvline(x = d_travel_near/(v_mean-v_sigma), color= 'gray', linestyle = 'dashed')
    plt.axvline(x = d_travel_near/(v_mean+3*v_sigma), color= 'gray', linestyle = 'dotted', label = r'$3\sigma$')
    plt.axvline(x = d_travel_near/(v_mean-3*v_sigma), color= 'gray', linestyle = 'dotted')
    plt.xlabel(r'Travel Time $[s]$'.format(), fontsize=text_fontsize)
    plt.ylabel("Probability", fontsize=text_fontsize)
    plt.title("Distribution of Time of Arrival from Tx near", fontsize=title_fontsize)
    plt.legend(bbox_to_anchor=(1,1), loc = "upper left", fontsize = text_fontsize)
    plt.text(0.155, 0.035, r'$d = {}\, m$'.format(d), fontsize=text_fontsize)
    plt.text(0.155, 0.02, r'$\Delta y = {}\, m$'.format(dy), fontsize=text_fontsize)
    
    #plt.savefig("./study_results/study5/Calc_diagrams/time_n.pdf", bbox_inches="tight")
    
    plt.show()

#%%

if study == 'study5_2' or study == 'study5':# or study == 'paper/2':# or study == 'study7':# or study == 'study6':
    
    if study == 'study7':
        name = sys.argv[3]
    else:
        name = sys.argv[1]
        
    order = apply_config_var['order']


    min_frame = 32
    max_frame = 38
    
    mid = off
    mad = frpsym+off+int(frpsym/3)
    
    # mad = frpsym+off+int(frpsym/2)
    
    # min_frame = mid
    # max_frame = mad
    
    picked_frame = frame
    

    # if dy < 0.1:
    #     dy = str(int(round(dy, 2)*100))
    #     dy = "0.0" + dy
    # else:
    #     dy = str(int(round(dy, 2)*100))
    #     dy = "0." +dy
    
    with open('study_results/{study}/Calc_diagrams/{name}_dist_near.txt'.format(study = study, name = name), 'r') as f:
        fr_n_diag = f.readline().split(";")
    
    fr_n_diag = np.array(fr_n_diag[:-1], dtype='float64')


    with open('study_results/{study}/Calc_diagrams/{name}_dist_far.txt'.format(study = study, name = name), 'r') as f:
        fr_f_diag = f.readline().split(";")
    
    fr_f_diag = np.array(fr_f_diag[:-1], dtype='float64')
    

    fr = np.arange(min_frame, max_frame+1, 1)
    
    
    with open('study_results/{study}/Calc_diagrams/full_{name}_dist_near.txt'.format(study = study, name = name), 'r') as f:
        fr_n_diag_full = f.readline().split(";")
    
    fr_n_diag_full = np.array(fr_n_diag_full[:-1], dtype='float64')


    with open('study_results/{study}/Calc_diagrams/full_{name}_dist_far.txt'.format(study = study, name = name), 'r') as f:
        fr_f_diag_full = f.readline().split(";")
    
    fr_f_diag_full = np.array(fr_f_diag_full[:-1], dtype='float64')
    
    fr_full = np.arange(0, 150, 1)
    
    with open('study_results/{study}/Calc_diagrams/{name}_diff_fn.txt'.format(study = study, name = name), 'r') as f:
        fr_diff_fn = f.readline().split(";")
    
    fr_diff_fn = np.array(fr_diff_fn[:-1], dtype='float64')

    with open('study_results/{study}/Calc_diagrams/full_{name}_diff_fn.txt'.format(study = study, name = name), 'r') as f:
        fr_diff_fn_full = f.readline().split(";")
    
    fr_diff_fn_full = np.array(fr_diff_fn_full[:-1], dtype='float64')
    
    
    with open('study_results/{study}/Calc_diagrams/{name}_diff_h.txt'.format(study = study, name = name), 'r') as f:
        fr_diff_h = f.readline().split(";")
    
    fr_diff_h = np.array(fr_diff_h[:-1], dtype='float64')

    with open('study_results/{study}/Calc_diagrams/full_{name}_diff_h.txt'.format(study = study, name = name), 'r') as f:
        fr_diff_h_full = f.readline().split(";")
    
    fr_diff_h_full = np.array(fr_diff_h_full[:-1], dtype='float64')
    
    
    
    
    fig = plt.figure()
    fig.set_figwidth(15)
    fig.set_figheight(10)
    plt.rc('xtick', labelsize=tick_fontsize)
    plt.rc('ytick', labelsize=tick_fontsize)
    
    plt.plot(fr_full[mid:mad], (fr_n_diag_full[mid:mad]+fr_f_diag_full[mid:mad]), label = "'11' - H", color = 'b', linewidth = 3)
    # plt.plot(fr, (fr_n_diag+fr_f_diag), label = "High [min-frame; max-frame]", color = 'darkblue', linewidth = 4)
    plt.plot(fr, (fr_n_diag+fr_f_diag), color = 'darkblue', linewidth = 4) 
    
    plt.plot(fr_full[mid:mad], fr_f_diag_full[mid:mad], label = "'10' - F", color = 'r', linewidth = 3)
    # plt.plot(fr, fr_f_diag, label = "Far [min-frame; max-frame]", color = 'firebrick', linewidth = 4)
    plt.plot(fr, fr_f_diag, color = 'firebrick', linewidth = 4)
    
    plt.plot(fr_full[mid:mad], fr_n_diag_full[mid:mad], label = "'01' - N", color = 'orange', linewidth = 3)
    # plt.plot(fr, fr_n_diag, label = "Near [min-frame; max-frame]", color = 'darkgoldenrod', linewidth = 4)
    plt.plot(fr, fr_n_diag, color = 'darkgoldenrod', linewidth = 4)
    
    

    plt.plot(fr_full[mid:mad], fr_diff_fn_full[mid:mad], label = "abs(F--N)", color = 'c', linewidth = 2)
    # plt.plot(fr, fr_diff, label = "abs(diff) [min-frame; max-frame]", color = 'teal', linewidth = 3)
    plt.plot(fr, fr_diff_fn, color = 'teal', linewidth = 3)
    
    if order == "HFNL":
        plt.plot(fr_full[mid:mad], fr_diff_h_full[mid:mad], label = "abs(H--\_\_$_1$)", color = 'c', linewidth = 2)
    elif order =="HNFL":
        plt.plot(fr_full[mid:mad], fr_diff_h_full[mid:mad], label = "abs(H--\_\_$_1$)", color = 'c', linewidth = 2)
    # plt.plot(fr, fr_diff, label = "abs(diff) [min-frame; max-frame]", color = 'teal', linewidth = 3)
    plt.plot(fr, fr_diff_h, color = 'teal', linewidth = 3)

   
    
    plt.axvline(min_frame, color = 'black', alpha = 0.8, label = "Min Frame", linestyle = 'dashed', linewidth = 3)
    plt.axvline(max_frame, color = 'black', alpha = 0.8, label = "Max Frame", linestyle = 'dotted', linewidth = 3)
    plt.axvline(picked_frame, color = 'black', label = "Sampling Frame {}".format(picked_frame), linewidth = 3, linestyle = '-')

    plt.grid(color = 'gray', axis = 'x')
    plt.xlabel("Image Frames since Symbol Start", fontsize=text_fontsize)
    plt.ylabel("Probability of Molecules", fontsize=text_fontsize)
    plt.title("Expected CIR Shapes", fontsize=title_fontsize)
    
    plt.legend(bbox_to_anchor=(1,1), loc = "upper left", fontsize = text_fontsize)
    plt.text(mad + 3, 0.035, r'$t_{sym} = {val}\, ms$'.format(sym = "{sym}", val = str(int(t_sym * 1000))), fontsize=text_fontsize)
    plt.text(mad + 3, 0.02, r'$\Delta t = 0.0$'.format(sym = "{sym}", val = str(int(t_sym * 1000))), fontsize=text_fontsize)
    plt.text(mad + 3, 0.005, r'$d = {}\, m$'.format(d), fontsize=text_fontsize)
    plt.text(mad + 3, -0.01, r'$\Delta y = {}\, m$'.format(dy), fontsize=text_fontsize)
        
    
    plt.xticks(np.arange(mid, mad, 2))
    
    #save = True
    if save:
        plt.savefig("./study_results/{study}/Calc_diagrams/complete_{sim}.pdf".format(study = study, sim = sys.argv[1]), bbox_inches="tight")
    
    plt.show()
    
    



#%% ITR

#Plot HFN with h,l,f,l,n,l
#put cutoff at symbol duration
#and put cutoff at symbol duration + t_ftr, v3sigma
#add values before and behind the cutoffs up. 

ITR = False

if ITR:
    p = 2.9

    fig = plt.figure()
    fig.set_figwidth(23)
    fig.set_figheight(15)
    plt.rc('xtick', labelsize=tick_fontsize+2)
    plt.rc('ytick', labelsize=tick_fontsize+2)

    i = 0
    while vals[i] == 0:
        i = i+1
    off = i-1

    h = np.zeros((int(np.ceil(frpsym*p))+1, int(n_bit/9)+1))
    f = np.zeros((int(np.ceil(frpsym*p))+1, int(n_bit/9)+1))
    n = np.zeros((int(np.ceil(frpsym*p))+1, int(n_bit/9)+1))
    l = np.zeros((int(np.ceil(frpsym*p))+1, int(n_bit/9)+1))

    h_count = 0
    f_count = 0
    n_count = 0
    l_count = 0

    for j in range(0, n_bit-3, 3):
        # print(codes[j%len(codes)])
        if codes[j%len(codes)] == 'h':
            for k in range(0, int(np.ceil(frpsym*p))+1):
                h[k][h_count] = vals[j*frpsym + off + k]
                
            h_count = h_count + 1 
            x = np.arange(off, off + int(np.ceil(p*frpsym)), 1)
            y = vals[j*frpsym + off: j*frpsym + off + int(np.ceil(frpsym*p))]
            plt.plot(x, y, color = "blue")
        elif codes[j%len(codes)] == 'f':
            for k in range(0, int(np.ceil(frpsym*p))+1):
                f[k][f_count] = vals[j*frpsym + off + k]
                
            f_count = f_count + 1 
            x = np.arange(off, off + int(np.ceil(p*frpsym)), 1)
            y = vals[j*frpsym + off: j*frpsym + off + int(np.ceil(frpsym*p))]
            plt.plot(x, y, color = "red")
        elif codes[j%len(codes)] == 'n':
            for k in range(0, int(np.ceil(frpsym*p))+1):
                n[k][n_count] = vals[j*frpsym + off + k]
                
            n_count = n_count + 1 
            x = np.arange(off, off + int(np.ceil(p*frpsym)), 1)
            y = vals[j*frpsym + off: j*frpsym + off + int(np.ceil(frpsym*p))]
            plt.plot(x, y, color = "orange")
            


    h_mean = []
    f_mean = []
    n_mean = []
    for fr in h:
        h_mean.append(np.mean(fr))
    for fr in f:
        f_mean.append(np.mean(fr))
    for fr in n:
        n_mean.append(np.mean(fr))
    

    x2 = np.arange(off, off + int(np.ceil(p*frpsym))+1, 1)
    plt.plot(x2, h_mean, color = 'cyan', label = "H mean", linewidth = 3)
    plt.plot(x2, f_mean, color = 'firebrick', label = "F mean", linewidth = 3)
    plt.plot(x2, n_mean, color = 'darkgoldenrod', label = "N mean", linewidth = 3)

    h_A_SYM = np.sum(h_mean[:frpsym])
    h_A_ISI = np.sum(h_mean[frpsym:])
    f_A_SYM = np.sum(f_mean[:frpsym])
    f_A_ISI = np.sum(f_mean[frpsym:])
    n_A_SYM = np.sum(n_mean[:frpsym])
    n_A_ISI = np.sum(n_mean[frpsym:])
    
    plt.annotate("$A_{SYM}(H) = $\n${val}$".format(SYM = "{SYM}", val = str(round(h_A_SYM, 2))), (off + frpsym*0.05, h_mean[int(frpsym*0.8)]*0.75), fontsize = tick_fontsize, color = "black")
    plt.annotate("$A_{ISI}(H) = $\n${val}$".format(ISI = "{ISI}", val = str(round(h_A_ISI, 2))), (off + frpsym*2.3, h_mean[int(frpsym*0.8)]*0.75), fontsize = tick_fontsize, color = "black")
    plt.annotate("$A_{SYM}(F) = $\n${val}$".format(SYM = "{SYM}", val = str(round(f_A_SYM, 2))), (off + frpsym*0.05, h_mean[int(frpsym*0.8)]*0.6), fontsize = tick_fontsize, color = "black")
    plt.annotate("$A_{ISI}(F) = $\n${val}$".format(ISI = "{ISI}", val = str(round(f_A_ISI, 2))), (off + frpsym*2.3, h_mean[int(frpsym*0.8)]*0.6), fontsize = tick_fontsize, color = "black")
    plt.annotate("$A_{SYM}(N) = $\n${val}$".format(SYM = "{SYM}", val = str(round(n_A_SYM, 2))), (off + frpsym*0.05, h_mean[int(frpsym*0.8)]*0.45), fontsize = tick_fontsize, color = "black")
    plt.annotate("$A_{ISI}(N) = $\n${val}$".format(ISI = "{ISI}", val = str(round(n_A_ISI, 2))), (off + frpsym*2.3, h_mean[int(frpsym*0.8)]*0.45), fontsize = tick_fontsize, color = "black")
    plt.text(off + frpsym*p * 1.02, h_mean[int(frpsym*0.8)]*0.75, "$ITR(H) = $\n${val}$".format(val = str(round(h_A_ISI/(h_A_SYM+h_A_ISI), 5))), fontsize = tick_fontsize, color = "black")
    plt.text(off + frpsym*p * 1.02, h_mean[int(frpsym*0.8)]*0.6, "$ITR(F) = $\n${val}$".format(val = str(round(f_A_ISI/(f_A_SYM+f_A_ISI), 5))), fontsize = tick_fontsize, color = "black")
    plt.text(off + frpsym*p * 1.02, h_mean[int(frpsym*0.8)]*0.45, "$ITR(N) = $\n${val}$".format(val = str(round(n_A_ISI/(n_A_SYM+n_A_ISI), 5))), fontsize = tick_fontsize, color = "black")
    plt.text((p*frpsym+off)*1.02, h_mean[int(frpsym*0.8)]*0.3, r"$t_{sym} = {val} \, ms$\\[1ex]off$_f = {offf}$\\[1ex]off$_n = {offn}$\\[1ex]$d = {d}$\\[1ex]$\Delta y = {dy}\, m$".format(sym = "{sym}", val = str(int(t_sym * 1000)), offf = round(off_f, 2), offn = round(off_n, 2), d = d, dy = dy), fontsize=text_fontsize-3)

    
    
    
    plt.axvline(off + frpsym, color = 'black', linewidth = 3)
    plt.annotate(r"$t_s$", xy = (off + frpsym, 0), fontsize = text_fontsize, xytext = (-5, -30), textcoords = 'offset points')
    plt.axvline(off + 2*frpsym, color = 'black', linewidth = 3, linestyle = "dashed")
    plt.annotate(r"$2t_s$", xy = (off + 2*frpsym, 0), fontsize = text_fontsize, xytext = (-10, -30), textcoords = 'offset points')
    plt.axvline(frpsym+frame, color = 'black', linewidth = 3, linestyle = "dotted")
    plt.annotate("Reconstruction\nFrame", xy = (frpsym+frame, h_mean[int(frpsym*0.8)]), fontsize = tick_fontsize, xytext = (-200, 0), textcoords = 'offset points')
    
    
    plt.xlabel("Frame", fontsize = text_fontsize)
    plt.ylabel("LIV", fontsize = text_fontsize)
    plt.title("ITR", fontsize = title_fontsize)
    
    plt.ylim(ymin = 0)
    plt.xlim(xmin = off, xmax = off+frpsym*p)
    
    
    plt.legend(bbox_to_anchor=(1,1), loc = "upper left", fontsize=text_fontsize)
    
    #save = True
    if save:
        plt.savefig("study_results/study7/ITR/{name}.pdf".format(name = sys.argv[1]), bbox_inches="tight")

    plt.show()

    #plot
    #means
    #add means from off to off+frpsym
    #add means from off+frpsym to liv[j] == 0


#%% MOL Eye

import statistics as stats

MOL_Eye = False

if MOL_Eye:
    fig = plt.figure()
    fig.set_figwidth(5)
    fig.set_figheight(8)
    plt.rc('xtick', labelsize=13)
    plt.rc('ytick', labelsize=13)
    ann_y_off = -6
    
    order = apply_config_var['order']
    
    print(sys.argv[1])
    print(order)
    
    x= frame
    
    h = []
    f = []
    n = []
    l = []
    
    for i in range(n_bit):
        liv = vals[(i*frpsym+frame)]
        
        group = codes[i%len(codes)]
        if group == 'h':
            h.append(liv)
        elif group == 'f':
            f.append(liv)
        elif group == 'n':
            n.append(liv)
        elif group == 'l':
            l.append(liv)
        else:
            raise Exception("code invalid")           
                
        plt.plot(x, liv, label = labels_fn(i), color= colors[i%len(colors)], marker = ".", ms = 12)
        
    
    
    h_mean = np.mean(h)
    h_stdev = stats.stdev(h)
    f_mean = np.mean(f)
    f_stdev = stats.stdev(f)
    n_mean = np.mean(n)
    n_stdev = stats.stdev(n)
    l_mean = np.mean(l)
    l_stdev = stats.stdev(l)
    
    # plt.plot(x, h_mean, label = "code '11' mean and stdev", color = "cyan", marker= "_")
    plt.plot(x, h_mean, color = "cyan", marker= "_", ms = 15)
    plt.plot(x, h_mean + h_stdev, color = "cyan", marker= "1", ms = 15)
    plt.plot(x, h_mean - h_stdev, color = "cyan", marker= "2", ms = 15)
    plt.annotate("STD(H) = " + str(round(h_stdev, 3)), (x, h_mean), xytext = (-125, ann_y_off), textcoords = 'offset points', fontsize = 13)
    plt.annotate("STD(F) = " + str(round(f_stdev, 3)), (x, f_mean), xytext = (-125, ann_y_off), textcoords = 'offset points', fontsize = 13)
    plt.annotate("STD(N) = " + str(round(n_stdev, 3)), (x, n_mean), xytext = (-125, ann_y_off), textcoords = 'offset points', fontsize = 13)
    plt.annotate("STD(L) = " + str(round(l_stdev, 3)), (x, l_mean), xytext = (-125, ann_y_off), textcoords = 'offset points', fontsize = 13)
    plt.annotate(r"$t_{sym} = {val} \, ms$\\[1ex]off$_f = {offf}$\\[1ex]off$_n = {offn}$\\[1ex]$d = {d}$\\[1ex]$\Delta y = {dy}\, m$".format(sym = "{sym}", val = str(int(t_sym * 1000)), offf = round(off_f, 2), offn = round(off_n, 2), d = d, dy = dy), (x, 0), xytext = (150, 70), textcoords = 'offset points', fontsize=13)
    # plt.axvline(x, h_mean - h_stdev, h_mean + h_stdev, color = "cyan", linestyle = "dashed")
    


    if order == "HFNL":
        plt.annotate("h(HF) = " + str(round(h_mean - f_mean, 2)), (x, (h_mean+f_mean)/2), xytext = (12, ann_y_off), textcoords = 'offset points', fontsize = 13)
        
        delta_hf = np.zeros((len(h), len(f)))
        for i in range(len(h)):
            for j in range(len(f)):
                delta_hf[i][j] = h[i] - f[j]

        delta_hf = delta_hf.flatten()

        CSNR_hf = np.mean(delta_hf)/stats.stdev(delta_hf)
        print(CSNR_hf)
        plt.annotate(r"CSNR(HF) $= {}$".format(round(CSNR_hf, 3)), (x, 0), xytext = (150, 250), textcoords = 'offset points', fontsize=13)
                
    elif order == "HNFL":
        plt.annotate("h(HN) = " + str(round(h_mean - n_mean, 2)), (x, (h_mean + n_mean)/2), xytext = (12, ann_y_off), textcoords = 'offset points', fontsize = 13)

        delta_hn = np.zeros((len(h), len(n)))
        for i in range(len(h)):
            for j in range(len(n)):
                delta_hn[i][j] = h[i] - n[j]

        delta_hn = delta_hn.flatten()

        CSNR_hn = np.mean(delta_hn)/stats.stdev(delta_hn)
        print(CSNR_hn)
        plt.annotate(r"CSNR(HN) $= {}$".format(round(CSNR_hn, 3)), (x, 0), xytext = (150, 250), textcoords = 'offset points', fontsize=13)
                
        # plt.axvline(x, n_mean, h_mean, label = "h(H|N) = " + str(round(h_mean - n_mean, 2)), color = "green", linestyle = "dotted")

    # plt.plot(x, f_mean, label = "code '10' mean and stdev", color = "firebrick", marker= "_")
    plt.plot(x, f_mean, color = "firebrick", marker= "_", ms = 15)
    plt.plot(x, f_mean + f_stdev, color = "firebrick", marker= "1", ms = 15)
    plt.plot(x, f_mean - f_stdev, color = "firebrick", marker= "2", ms = 15)
    # plt.axvline(x, f_mean - f_stdev, f_mean + f_stdev, color = "firebrick", linestyle = "dashed")

    if order == "HFNL":
        plt.annotate("h(FN) = " + str(round(f_mean - n_mean, 2)), (x, (f_mean+n_mean)/2), xytext = (12, ann_y_off), textcoords = 'offset points', fontsize = 13)

        delta_fn = np.zeros((len(f), len(n)))
        for i in range(len(f)):
            for j in range(len(n)):
                delta_fn[i][j] = f[i] - n[j]
            
        delta_fn = delta_fn.flatten()

        CSNR_fn = np.mean(delta_fn)/stats.stdev(delta_fn)
        print(CSNR_fn)
        plt.annotate(r"CSNR(FN) $= {}$".format(round(CSNR_fn, 3)), (x, 0), xytext = (150, 200), textcoords = 'offset points', fontsize=13)
                

        # plt.axvline(x, n_mean, f_mean, label = "h(F|N) = " + str(round(f_mean - n_mean, 2)), color = "tomato", linestyle = (0,(1,5)))
    elif order == "HNFL":
        plt.annotate("h(NF) = " + str(round(n_mean - f_mean, 2)), (x, (n_mean+f_mean)/2), xytext = (12, ann_y_off), textcoords = 'offset points', fontsize = 13)

        delta_nf = np.zeros((len(n), len(f)))
        for i in range(len(n)):
            for j in range(len(f)):
                delta_nf[i][j] = n[i] - f[j]

        delta_nf = delta_nf.flatten()

        CSNR_nf = np.mean(delta_nf)/stats.stdev(delta_nf)
        print(CSNR_nf)
        plt.annotate(r"CSNR(NF) $= {}$".format(round(CSNR_nf, 3)), (x, 0), xytext = (150, 200), textcoords = 'offset points', fontsize=13)
                

        # plt.axvline(x, f_mean, n_mean, label = "h(N|F) = " + str(round(n_mean - f_mean, 2)), color = "tomato", linestyle = (0,(1,5)))

    # plt.plot(x, n_mean, label = "code '01' mean and stdev", color = "darkgoldenrod", marker= "_")
    plt.plot(x, n_mean, color = "darkgoldenrod", marker= "_", ms = 15)
    plt.plot(x, n_mean + n_stdev, color = "darkgoldenrod", marker= "1", ms = 15)
    plt.plot(x, n_mean - n_stdev, color = "darkgoldenrod", marker= "2", ms = 15)
    # plt.axvline(x, n_mean - n_stdev, n_mean + n_stdev, color = "darkgoldenrod", linestyle = "dashed")

    if order == "HFNL":
        plt.annotate("h(NL) = " + str(round(n_mean - l_mean, 2)), (x, (n_mean + l_mean)/2), xytext = (12, ann_y_off), textcoords = 'offset points', fontsize = 13)

        delta_nl = np.zeros((len(n), len(l)))
        for i in range(len(n)):
            for j in range(len(l)):
                delta_nl[i][j] = n[i] - l[j]

        delta_nl = delta_nl.flatten()

        CSNR_nl = np.mean(delta_nl)/stats.stdev(delta_nl)
        print(CSNR_nl)
        plt.annotate(r"CSNR(NL) $= {}$".format(round(CSNR_nl, 3)), (x, 0), xytext = (150, 150), textcoords = 'offset points', fontsize=13)
                

        # plt.axvline(x, l_mean, n_mean, label = "h(N|L) = " + str(round(n_mean - l_mean, 2)), color = "tan", linestyle = (0,(1,10)))
    elif order == "HNFL":
        plt.annotate("h(FL) = " + str(round(f_mean - l_mean, 2)), (x, (f_mean + l_mean)/2), xytext = (12, ann_y_off), textcoords = 'offset points', fontsize = 13)

        delta_fl = np.zeros((len(f), len(l)))
        for i in range(len(f)):
            for j in range(len(l)):
                delta_fl[i][j] = f[i] - l[j]

        delta_fl = delta_fl.flatten()

        CSNR_fl = np.mean(delta_fl)/stats.stdev(delta_fl)
        print(CSNR_fl)
        plt.annotate(r"CSNR(FL) $= {}$".format(round(CSNR_fl, 3)), (x, 0), xytext = (150, 150), textcoords = 'offset points', fontsize=13)
                

        # plt.axvline(x, l_mean, f_mean, label = "h(F|L) = " + str(round(f_mean - l_mean, 2)), color = "tan", linestyle = (0,(1,10)))

    # plt.plot(x, l_mean, label = "code '00' mean and stdev", color = "black", marker= "_")
    plt.plot(x, l_mean, color = "black", marker= "_", ms = 15)
    plt.plot(x, l_mean + l_stdev, color = "black", marker= "1", ms = 15)
    plt.plot(x, l_mean - l_stdev, color = "black", marker= "2", ms = 15)
    # plt.axvline(x, l_mean - l_stdev, l_mean + l_stdev, color = "black", linestyle = "dashed")

    
    plt.legend(bbox_to_anchor=(1,1), loc = "upper left", fontsize=13)
    
    
    #plt.xlabel('Sim2Vid', fontsize=18)
    #plt.xlabel('SCC', fontsize=18)
    plt.xlabel('Reconstruction Frame', fontsize=18)
    
    plt.ylabel('Relative LIV', fontsize=18)
    
    plt.title('Adapted MOL-Eye', fontsize=20)
    plt.xticks([x])
    
    save = True
    if save:
        plt.savefig("./study_results/{study}/{version}/MOL-EYE_{name}.pdf".format(study = study, version = version, name = sys.argv[1]), bbox_inches="tight")
    
    plt.show()




#%% Std Deviation Thresholds


import csv

if study == 'study8':
    
    dir_8 = "study_results/study8/Threshold_Deviation/" 
    
    minmax = "0"
    quartile = "1"
    median = "2"
    arithmean = "3"
    
    calcs = [(minmax, "MinMax"), (quartile, "Quartile"), (median, "Median"), (arithmean, "ArithMean")]
    
    cases = list(range(1, 10, 1))
    # cases = [1]
    
    for (calc, c_name) in calcs:
        calc_BER_n_l = []
        calc_BER_f_l = []
        calc_BER_n_a = []
        calc_BER_f_a = []
        
        for case in cases:
    
            filename = dir_8 + "A_" + str(case) + "_" + calc + " copy.csv"
            
            
            B = []
            M = []
            S = []
            BER_n_l = []
            BER_f_l = []
            BER_n_a = []
            BER_f_a = []
            
            # Quote https://www.geeksforgeeks.org/working-csv-files-python/
            
            fields = []
            rows = []
            
            with open(filename, 'r') as csvfile:
                csvreader = csv.reader(csvfile)
                fields = next(csvreader)
            # Quote end
                i = 0
                for row in csvreader:
                    if i % 2 == 0:
                        B.append(row[2])
                        M.append(row[3])
                        S.append(row[4])
                        BER_n_l.append(row[8])
                        BER_f_l.append(row[13])
                    else:
                        BER_n_a.append(row[8])
                        BER_f_a.append(row[13])
                    i = i + 1
                    
            B = np.array(B, dtype='float64')
            M = np.array(M, dtype='float64')
            S = np.array(S, dtype='float64')
            BER_n_l = np.array(BER_n_l, dtype='float64')
            BER_f_l = np.array(BER_f_l, dtype='float64')
            BER_n_a = np.array(BER_n_a, dtype='float64')
            BER_f_a = np.array(BER_f_a, dtype='float64')

            print(filename)
            # print("Mean:")
            # print("BER n l " + str(np.mean(BER_n_l)))
            # print("BER f l " + str(np.mean(BER_f_l)))
            print("BER n a " + str(np.mean(BER_n_a)))
            print("BER f a " + str(np.mean(BER_f_a)))
            # print("STD:")
            # print("BER n l " + str(stats.stdev(BER_n_l)))
            # print("BER f l " + str(stats.stdev(BER_f_l)))
            # print("BER n a " + str(stats.stdev(BER_n_a)))
            # print("BER f a " + str(stats.stdev(BER_f_a)))
            # print("BER N Apply Max = " + str(max(BER_n_a)))
            # print("BER F Apply Max = " + str(max(BER_f_a)))

            
            calc_BER_n_l.append(np.mean(BER_n_l))
            calc_BER_f_l.append(np.mean(BER_f_l))
            calc_BER_n_a.append(np.mean(BER_n_a))
            calc_BER_f_a.append(np.mean(BER_f_a))
                
        print(c_name)

        
                
        
#%%

def BER_arr(filename):
    BER_n = [] #append all BER from apply
    BER_f = [] 
    
    fields = []
    
    with open(filename, 'r') as csvfile:
        csvreader = csv.reader(csvfile)
        fields = next(csvreader)
        
        i = 0
        for row in csvreader:
            if i % 2 == 1:
                BER_n.append(row[8])
                BER_f.append(row[13])
            i = i + 1
            
    BER_n = np.array(BER_n, dtype='float64')
    BER_f = np.array(BER_f, dtype='float64')
    return(BER_n, BER_f)


def compare_lines(i, nf, dy, fr):
    if nf == "N":
        color_arr = ["orange", "darkkhaki", "darkgoldenrod", "burlywood", "yellowgreen"]
        color = color_arr[i]
    else:
        color_arr = ["red", "deeppink", "maroon", "lightcoral", "mediumorchid"]
        color = color_arr[i]
    
    if dy == 0:
        style_arr = ["dashed", "dotted", "dotted", "dotted", "dotted"]
        style = style_arr[i]
    else:
        style_arr = ["-", "--", "--", "--", "--"]
        style = style_arr[i]
        
    if i == 0:
        width = 3
    else:
        width = 2
    
    if fr == "":
        label = nf + ", $\Delta$y = " +str(dy)+ " cm"
    else:
        # label = nf + ", dy = " +str(dy)+ " cm, \nfr_off = "+fr
        label = "fr\_off = "+fr
    
    
    return(color, style, width, label, 1)
        
    
        
    
    
    
    

#%%
import csv

if study == 'paper/1' or study == 'paper/2':
    
    zoom = True
    zoom = False
    v = '1'
    v = '2'
    
    fig = plt.figure()
    fig.set_figwidth(13)
    fig.set_figheight(6)
    plt.rc('xtick', labelsize=22)
    plt.rc('ytick', labelsize=22)
    
    #filename = "study_results/" + study + "/BER/" + sys.argv[1] + "_0.csv"
    
    cases_n = ["0.00", "0.05", "0.10", "0.15", "0.20", "0.25", "0.30", "0.35", "0.40", "0.45", "0.50", "0.55", "0.60"]
    cases_n.reverse()
    cases_f = ["0.05", "0.10", "0.15", "0.20", "0.25", "0.30", "0.35", "0.40", "0.45", "0.50", "0.55", "0.60"]

    BER_mean_n_1 = []
    BER_mean_f_1 = []
    
    BER_mean_n_2 = []
    BER_mean_f_2 = []

    for c in cases_n:
        filename_1 = "study_results/paper/1/BER/A_"+v+"_off_f_" + c + "_0.csv"
        filename_2 = "study_results/paper/2/BER/A_"+v+"_off_n_" + c + "_0.csv"
        
        (BER_n_1, BER_f_1) = BER_arr(filename_1) 
        (BER_n_2, BER_f_2) = BER_arr(filename_2) 
        
        BER_mean_n_1.append(np.mean(BER_n_1))
        BER_mean_f_1.append(np.mean(BER_f_1))
        BER_mean_n_2.append(np.mean(BER_n_2))
        BER_mean_f_2.append(np.mean(BER_f_2))
        
    for c in cases_f:
        filename_1 = "study_results/paper/1/BER/A_"+v+"_off_f_" + c + "_0.csv"
        filename_2 = "study_results/paper/2/BER/A_"+v+"_off_f_" + c + "_0.csv"
        
        (BER_n_1, BER_f_1) = BER_arr(filename_1) 
        (BER_n_2, BER_f_2) = BER_arr(filename_2) 
        
        BER_mean_n_1.append(np.mean(BER_n_1))
        BER_mean_f_1.append(np.mean(BER_f_1))
        BER_mean_n_2.append(np.mean(BER_n_2))
        BER_mean_f_2.append(np.mean(BER_f_2))

        
    # plt.hlines(min(BER_mean_f_1), -0.6, 0.6, color = 'black', linestyle = 'dashed', label = r'min BER, $\Delta y = 0$ cm')
    # plt.hlines(min(BER_mean_n_2), -0.6, 0.6, color = 'black', label = r'min BER, $\Delta y = 10$ cm')

    cases = np.arange(-0.60, 0.61, 0.05)    
    
    

    plt.plot(cases, BER_mean_n_1, color = "orange", linestyle = 'dashed', label = r'N, $\Delta y = 0$ cm')
    plt.plot(cases, BER_mean_f_1, color = "red", linestyle = 'dashed', label = r'F, $\Delta y = 0$ cm')
    plt.plot(cases, BER_mean_n_2, color = "orange", label = r'N, $\Delta y = 10$ cm')
    plt.plot(cases, BER_mean_f_2, color = "red", label = r'F, $\Delta y = 10$ cm')
    
    if zoom == True:
        if v == '1': # t sym = 75ms
            plt.ylim(0.0, 0.05)
            plt.xlim(-0.6, -0.2)
            
             # minimum with dy = 0cm
            circle_rad = 20  # This is the radius, in points
            plt.plot(-0.4, (BER_mean_n_1[4]+BER_mean_f_1[4])/2, 'o',
                ms=circle_rad * 2, mec='b', mfc='none', mew=2)
            plt.annotate("BER(N) = {n}\nBER(F) = {f}".format(n = BER_mean_n_1[4].round(4), f = BER_mean_f_1[4].round(4)), xy=(-0.4, (BER_mean_n_1[4]+BER_mean_f_1[4])/2), xytext=(-108, 80),
                    textcoords='offset points',
                    color='b', size='xx-large',
                    arrowprops=dict(
                        arrowstyle='simple,tail_width=0.3,head_width=0.8,head_length=0.8',
                        facecolor='b', shrinkB=circle_rad * 1.2)
                    ) 
            
            #minimum with dy = 10cm
            circle_rad = 17  # This is the radius, in points
            plt.plot(-0.5, (BER_mean_n_2[2]+BER_mean_f_2[2])/2, 'o',
                ms=circle_rad * 2, mec='b', mfc='none', mew=2)
            plt.annotate("BER(F) = {f}\nBER(N) = {n}".format(n = BER_mean_n_2[2].round(4), f = BER_mean_f_2[2].round(4)), xy=(-0.5, (BER_mean_n_2[2]+BER_mean_f_2[2])/2), xytext=(-160, 50),
                    textcoords='offset points',
                    color='b', size='xx-large',
                    arrowprops=dict(
                        arrowstyle='simple,tail_width=0.3,head_width=0.8,head_length=0.8',
                        facecolor='b', shrinkB=circle_rad * 1.2)
                    )
            
        else:
            plt.ylim(0.0, 0.01)
            plt.xlim(-0.45, -0.2)
            
            #from https://stackoverflow.com/questions/37489874/how-do-i-put-a-circle-with-annotation-in-matplotlib
             # minimum with dy = 0cm
            circle_rad = 40  # This is the radius, in points
            plt.plot(-0.25, (BER_mean_n_1[7]+BER_mean_f_1[7])/2, 'o',
                ms=circle_rad * 2, mec='b', mfc='none', mew=2)
            plt.annotate("BER(N) = {n}\nBER(F) = {f}".format(n = BER_mean_n_1[7].round(4), f = BER_mean_f_1[7].round(4)), xy=(-0.25, (BER_mean_n_1[7]+BER_mean_f_1[7])/2), xytext=(-15, 90),
                    textcoords='offset points',
                    color='b', size='xx-large',
                    arrowprops=dict(
                        arrowstyle='simple,tail_width=0.3,head_width=0.8,head_length=0.8',
                        facecolor='b', shrinkB=circle_rad * 1.2)
                    ) 
            
            #minimum with dy = 10cm
            circle_rad = 17  # This is the radius, in points
            plt.plot(-0.4, (BER_mean_n_2[4]+BER_mean_f_2[4])/2, 'o',
                ms=circle_rad * 2, mec='b', mfc='none', mew=2)
            plt.annotate("BER(F) = {f}\nBER(N) = {n}".format(n = BER_mean_n_2[4].round(4), f = BER_mean_f_2[4].round(4)), xy=(-0.4, (BER_mean_n_2[4]+BER_mean_f_2[4])/2), xytext=(-15, 60),
                    textcoords='offset points',
                    color='b', size='xx-large',
                    arrowprops=dict(
                        arrowstyle='simple,tail_width=0.3,head_width=0.8,head_length=0.8',
                        facecolor='b', shrinkB=circle_rad * 1.2)
                    )
    else:
        plt.ylim(0.0, 0.305)
        plt.xlim(-0.6, 0.6)
    
    
    
    
        
    plt.legend(loc = "upper left", fontsize=22) #bbox_to_anchor=(1,1), 
    # if v == '1':
    #     plt.title("BER by Timing Offset, d = 280 cm, t(sym) = 75 ms", fontsize = 25)
    # else:
    #     plt.title("BER by Timing Offset, d = 280 cm, t(sym) = 100 ms", fontsize = 25)
    
    if zoom == True:
        True
        #plt.title("Zoomed in", fontsize = 25)
    else:
        plt.title("BER by Timing Offset, d = 280 cm", fontsize = 25)
        
    plt.ylabel("BER", fontsize=22)
    plt.xlabel("Timing Offset Tx Far", fontsize=22)
    
    if zoom == True:
        plt.savefig("./study_results/paper/BER280_zoom"+v+".pdf", bbox_inches="tight")
    else:
        plt.savefig("./study_results/paper/BER280"+v+".pdf", bbox_inches="tight")
    
    plt.show()
        
        
    
    
#%% Frame Choice Comparison

import csv
fr_off_compare = True

if (study == 'paper/1' or study == 'paper/2') and fr_off_compare == True:
    
    zoom = False
    zoom = True
    v = '1'
    v = '2'
    fr_off=["","-1","+1","-3","+3"]
    
    fig = plt.figure()
    fig.set_figwidth(13)
    fig.set_figheight(6)
    plt.rc('xtick', labelsize=20)
    plt.rc('ytick', labelsize=20)
    
    #filename = "study_results/" + study + "/BER/" + sys.argv[1] + "_0.csv"
    
    cases_n = ["0.00", "0.05", "0.10", "0.15", "0.20", "0.25", "0.30", "0.35", "0.40", "0.45", "0.50", "0.55", "0.60"]
    cases_n.reverse()
    cases_f = ["0.05", "0.10", "0.15", "0.20", "0.25", "0.30", "0.35", "0.40", "0.45", "0.50", "0.55", "0.60"]

    i = 0    

    for fr in fr_off:

        BER_mean_n_1 = []
        BER_mean_f_1 = []
        
        BER_mean_n_2 = []
        BER_mean_f_2 = []
    
        for c in cases_n:
            filename_1 = "study_results/paper/1/BER/A_"+v+fr+"_off_f_" + c + "_0.csv"
            filename_2 = "study_results/paper/2/BER/A_"+v+fr+"_off_n_" + c + "_0.csv"
            
            (BER_n_1, BER_f_1) = BER_arr(filename_1) 
            (BER_n_2, BER_f_2) = BER_arr(filename_2) 
            
            BER_mean_n_1.append(np.mean(BER_n_1))
            BER_mean_f_1.append(np.mean(BER_f_1))
            BER_mean_n_2.append(np.mean(BER_n_2))
            BER_mean_f_2.append(np.mean(BER_f_2))
            
        for c in cases_f:
            filename_1 = "study_results/paper/1/BER/A_"+v+fr+"_off_f_" + c + "_0.csv"
            filename_2 = "study_results/paper/2/BER/A_"+v+fr+"_off_f_" + c + "_0.csv"
            
            (BER_n_1, BER_f_1) = BER_arr(filename_1) 
            (BER_n_2, BER_f_2) = BER_arr(filename_2) 
            
            BER_mean_n_1.append(np.mean(BER_n_1))
            BER_mean_f_1.append(np.mean(BER_f_1))
            BER_mean_n_2.append(np.mean(BER_n_2))
            BER_mean_f_2.append(np.mean(BER_f_2))
    
            
        # plt.hlines(min(BER_mean_f_1), -0.6, 0.6, color = 'black', linestyle = 'dashed', label = r'min BER, $\Delta y = 0$ cm')
        # plt.hlines(min(BER_mean_n_2), -0.6, 0.6, color = 'black', label = r'min BER, $\Delta y = 10$ cm')
    
        cases = np.arange(-0.60, 0.61, 0.05)    
        
        (l_color_n_0, l_style_n_0, l_width_n_0, l_label_n_0, l_alpha_n_0) = compare_lines(i, "N", 0, fr)
        (l_color_n_10, l_style_n_10, l_width_n_10, l_label_n_10, l_alpha_n_10) = compare_lines(i, "N", 10, fr)
        (l_color_f_0, l_style_f_0, l_width_f_0, l_label_f_0, l_alpha_f_0) = compare_lines(i, "F", 0, fr)
        (l_color_f_10, l_style_f_10, l_width_f_10, l_label_f_10, l_alpha_f_10) = compare_lines(i, "F", 10, fr)
    
        # plt.plot(cases, BER_mean_n_1, color = l_color_n_0, linestyle = l_style_n_0, linewidth = l_width_n_0, label = l_label_n_0)
        # plt.plot(cases, BER_mean_f_1, color = l_color_f_0, linestyle = l_style_f_0, linewidth = l_width_f_0, label = l_label_f_0)
        # plt.plot(cases, BER_mean_n_2, color = l_color_n_10, linestyle = l_style_n_10, linewidth = l_width_n_10, label = l_label_n_10)
        plt.plot(cases, BER_mean_f_2, color = l_color_f_10, linestyle = l_style_f_10, linewidth = l_width_f_10, label = l_label_f_10)
        
        if zoom == True:
            if v == '1': # t sym = 75ms
                plt.ylim(0.0, 0.05)
                plt.xlim(-0.6, -0.3)
                
                # for f2
                
                # if fr == "+1":
                #     plt.annotate("+1 min.: "+str(BER_mean_f_2[1].round(4)),
                #         xy = (-0.55, BER_mean_f_2[1]), 
                #         xytext = (-80, -20), 
                #         textcoords='offset points', 
                #         color = "black", 
                #         size = 'xx-large'
                #         )
                #     plt.vlines(-0.55, 0, 0.05, color = "gray")
                    
                # if fr == "":
                #     plt.annotate("min.: "+str(BER_mean_f_2[2].round(4)),
                #         xy = (-0.50, BER_mean_f_2[1]), 
                #         xytext = (5, -20), 
                #         textcoords='offset points', 
                #         color = "black", 
                #         size = 'xx-large'
                #         )
                #     plt.vlines(-0.50, 0, 0.05, color = "gray")
                    
                ################    
                
                # for n2
                
                # if fr == "+3":
                #     plt.annotate("+3 min.: "+str(BER_mean_n_2[0].round(4)),
                #         xy = (-0.6, BER_mean_n_2[0]), 
                #         xytext = (8, 30), 
                #         textcoords='offset points', 
                #         color = "black", 
                #         size = 'xx-large'
                #         )
                #     plt.vlines(-0.599, 0, 0.05, color = "gray")
                    
                # if fr == "":
                #     plt.annotate("min.: "+str(BER_mean_n_2[2].round(4)),
                #         xy = (-0.50, BER_mean_n_2[1]), 
                #         xytext = (5, -20), 
                #         textcoords='offset points', 
                #         color = "black", 
                #         size = 'xx-large'
                #         )
                #     plt.vlines(-0.50, 0, 0.05, color = "gray")
                    
                #################
                    
                # for f1
                
                # if fr == "+3":
                #     plt.annotate("+3 min.: "+str(BER_mean_f_1[2].round(4)),
                #         xy = (-0.5, BER_mean_f_1[2]), 
                #         xytext = (-77, -20), 
                #         textcoords='offset points', 
                #         color = "black", 
                #         size = 'xx-large'
                #         )
                #     plt.vlines(-0.5, 0, 0.05, color = "gray")
                    
                # if fr == "":
                #     plt.annotate("min.: "+str(BER_mean_f_1[5].round(4)),
                #         xy = (-0.35, BER_mean_f_1[5]), 
                #         xytext = (5, -20), 
                #         textcoords='offset points', 
                #         color = "black", 
                #         size = 'xx-large'
                #         )
                #     plt.vlines(-0.35, 0, 0.05, color = "gray")
                
                #####################
                    
                # for n1
                
                # if fr == "+1":
                #     plt.annotate("+1 min.: "+str(BER_mean_n_1[4].round(4)),
                #         xy = (-0.4, BER_mean_n_1[4]), 
                #         xytext = (-77, -30), 
                #         textcoords='offset points', 
                #         color = "black", 
                #         size = 'xx-large'
                #         )
                #     plt.vlines(-0.4, 0, 0.05, color = "gray")
                    
                # if fr == "":
                #     plt.annotate("min.: "+str(BER_mean_n_1[5].round(4)),
                #         xy = (-0.35, BER_mean_n_1[5]), 
                #         xytext = (5, -30), 
                #         textcoords='offset points', 
                #         color = "black", 
                #         size = 'xx-large'
                #         )
                #     plt.vlines(-0.35, 0, 0.05, color = "gray")
                
                ##############################
                    
                                 
                
            else:
                plt.ylim(0.0, 0.005)
                # plt.ylim(0.0, 0.01)
                plt.xlim(-0.5, -0.25)
                # plt.xlim(-0.4, -0.2)
                
                 # for f2
                
                if fr == "-1":
                    plt.annotate("-1: "+str(BER_mean_f_2[4].round(5)),
                        xy = (-0.4, BER_mean_f_2[4]), 
                        xytext = (8, 30), 
                        textcoords='offset points', 
                        color = "black", 
                        size = 'xx-large'
                        )
                    plt.vlines(-0.4, 0, 0.05, color = "gray")
                    
                if fr == "":
                    plt.annotate("+-0: "+str(BER_mean_f_2[4].round(5)),
                        xy = (-0.40, BER_mean_f_2[4]), 
                        xytext = (8, -25), 
                        textcoords='offset points', 
                        color = "black", 
                        size = 'xx-large'
                        )
                    plt.vlines(-0.40, 0, 0.05, color = "gray")
                    
                ################    
                
                # for n2
                
                # if fr == "+1":
                #     plt.annotate("+1: "+str(BER_mean_n_2[4].round(4)),
                #         xy = (-0.4, BER_mean_n_2[4]), 
                #         xytext = (8, -8), 
                #         textcoords='offset points', 
                #         color = "black", 
                #         size = 'xx-large'
                #         )
                #     plt.vlines(-0.4, 0, 0.05, color = "gray")
                    
                # if fr == "":
                #     plt.annotate("+-0: "+str(BER_mean_n_2[4].round(4)),
                #         xy = (-0.4, BER_mean_n_2[4]), 
                #         xytext = (8, 40), 
                #         textcoords='offset points', 
                #         color = "black", 
                #         size = 'xx-large'
                #         )
                #     plt.vlines(-0.40, 0, 0.05, color = "gray")
                    
                #################
                    
                # for f1
                
                # if fr == "-1":
                #     plt.annotate("-1: "+str(BER_mean_f_1[7].round(4)),
                #         xy = (-0.25, BER_mean_f_1[7]), 
                #         xytext = (10, 0), 
                #         textcoords='offset points', 
                #         color = "black", 
                #         size = 'xx-large'
                #         )
                #     plt.vlines(-0.25, 0, 0.05, color = "gray")
                    
                # if fr == "":
                #     plt.annotate("+-0: "+str(BER_mean_f_1[7].round(4)),
                #         xy = (-0.25, BER_mean_f_1[7]), 
                #         xytext = (10, 0), 
                #         textcoords='offset points', 
                #         color = "black", 
                #         size = 'xx-large'
                #         )
                #     plt.vlines(-0.25, 0, 0.05, color = "gray")
                
                #####################
                    
                # for n1
                
                # if fr == "-1":
                #     plt.annotate("-1: "+str(BER_mean_n_1[7].round(4)),
                #         xy = (-0.25, BER_mean_n_1[7]), 
                #         xytext = (8, 0), 
                #         textcoords='offset points', 
                #         color = "black", 
                #         size = 'xx-large'
                #         )
                #     plt.vlines(-0.25, 0, 0.05, color = "gray")
                
                # if fr == "-3":
                #     plt.annotate("-3: "+str(BER_mean_n_1[6].round(4)),
                #         xy = (-0.3, BER_mean_n_1[6]), 
                #         xytext = (-100, -10), 
                #         textcoords='offset points', 
                #         color = "black", 
                #         size = 'xx-large'
                #         )
                #     plt.vlines(-0.3, 0, 0.05, color = "gray")
                    
                # if fr == "":
                #     plt.annotate("+-0: "+str(BER_mean_n_1[7].round(4)),
                #         xy = (-0.25, BER_mean_n_1[7]), 
                #         xytext = (8, -20), 
                #         textcoords='offset points', 
                #         color = "black", 
                #         size = 'xx-large'
                #         )
                #     plt.vlines(-0.25, 0, 0.05, color = "gray")
                
                ##############################
                

        else:
            plt.ylim(0.0, 0.305)
            plt.xlim(-0.6, 0.6)
        
        i = i +1
        
        
    if zoom == True:        
        # plt.legend(loc = "lower right", fontsize = 22)
        plt.legend(loc = "upper left", fontsize = 22)
        # plt.legend(loc = "lower left", fontsize = 22)
    else:
        plt.legend(loc = "upper left", fontsize=22)#, bbox_to_anchor=(1,1)) 
        
    
    # if v == '1':
    #     plt.title("BER by Timing Offset, d = 280 cm, t(sym) = 75 ms", fontsize = 25)
    # else:
    #     plt.title("BER by Timing Offset, d = 280 cm, t(sym) = 100 ms", fontsize = 25)
    
    if zoom == True:
        True
        #plt.title("Zoomed in", fontsize = 25)
    else:
        # plt.title("BER Frame Choice Comparison, t(sym) = 75ms", fontsize = 25)
        plt.title("BER Frame Choice Comparison, t(sym) = 100ms", fontsize = 25)
        
    plt.ylabel("BER", fontsize=22)
    plt.xlabel("Timing Offset Tx Far", fontsize=22)
    
    save = True
    
    if zoom == True and save == True:
        plt.savefig("./study_results/paper/BER280_zoom"+v+"fr_off_FDY.pdf", bbox_inches="tight")
        # plt.savefig("./study_results/paper/BER280_zoom"+v+"fr_off_NDY.pdf", bbox_inches="tight")
        # plt.savefig("./study_results/paper/BER280_zoom"+v+"fr_off_F.pdf", bbox_inches="tight")
        # plt.savefig("./study_results/paper/BER280_zoom"+v+"fr_off_N.pdf", bbox_inches="tight")
    else:
        plt.savefig("./study_results/paper/BER280"+v+"fr_off_FDY.pdf", bbox_inches="tight")
        # plt.savefig("./study_results/paper/BER280"+v+"fr_off_NDY.pdf", bbox_inches="tight")
        # plt.savefig("./study_results/paper/BER280"+v+"fr_off_F.pdf", bbox_inches="tight")
        # plt.savefig("./study_results/paper/BER280"+v+"fr_off_N.pdf", bbox_inches="tight")
    
    plt.show()



