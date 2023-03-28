#[doc = "Register `ASCR1` reader"]
pub struct R(crate::R<ASCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASCR1` writer"]
pub struct W(crate::W<ASCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ASCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0GR1_1` reader - Analog switch control"]
pub type CH0GR1_1_R = crate::BitReader<bool>;
#[doc = "Field `CH0GR1_1` writer - Analog switch control"]
pub type CH0GR1_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR1_SPEC, bool, O>;
#[doc = "Field `CH1GR1_2` reader - Analog switch control"]
pub type CH1GR1_2_R = crate::BitReader<bool>;
#[doc = "Field `CH1GR1_2` writer - Analog switch control"]
pub type CH1GR1_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR1_SPEC, bool, O>;
#[doc = "Field `CH2GR1_3` reader - Analog switch control"]
pub type CH2GR1_3_R = crate::BitReader<bool>;
#[doc = "Field `CH2GR1_3` writer - Analog switch control"]
pub type CH2GR1_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR1_SPEC, bool, O>;
#[doc = "Field `CH3GR1_4` reader - Analog switch control"]
pub type CH3GR1_4_R = crate::BitReader<bool>;
#[doc = "Field `CH3GR1_4` writer - Analog switch control"]
pub type CH3GR1_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR1_SPEC, bool, O>;
#[doc = "Field `CH31GR11_5` reader - Analog switch control"]
pub type CH31GR11_5_R = crate::BitReader<bool>;
#[doc = "Field `CH31GR11_5` writer - Analog switch control"]
pub type CH31GR11_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR1_SPEC, bool, O>;
#[doc = "Field `COMP1_SW1` reader - Comparator 1 analog switch"]
pub type COMP1_SW1_R = crate::BitReader<bool>;
#[doc = "Field `COMP1_SW1` writer - Comparator 1 analog switch"]
pub type COMP1_SW1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR1_SPEC, bool, O>;
#[doc = "Field `CH6GR2_1` reader - Analog switch control"]
pub type CH6GR2_1_R = crate::BitReader<bool>;
#[doc = "Field `CH6GR2_1` writer - Analog switch control"]
pub type CH6GR2_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR1_SPEC, bool, O>;
#[doc = "Field `CH7GR2_2` reader - Analog switch control"]
pub type CH7GR2_2_R = crate::BitReader<bool>;
#[doc = "Field `CH7GR2_2` writer - Analog switch control"]
pub type CH7GR2_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR1_SPEC, bool, O>;
#[doc = "Field `CH8GR3_1` reader - Analog switch control"]
pub type CH8GR3_1_R = crate::BitReader<bool>;
#[doc = "Field `CH8GR3_1` writer - Analog switch control"]
pub type CH8GR3_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR1_SPEC, bool, O>;
#[doc = "Field `CH9GR3_2` reader - Analog switch control"]
pub type CH9GR3_2_R = crate::BitReader<bool>;
#[doc = "Field `CH9GR3_2` writer - Analog switch control"]
pub type CH9GR3_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR1_SPEC, bool, O>;
#[doc = "Field `CH10GR8_1` reader - Analog switch control"]
pub type CH10GR8_1_R = crate::BitReader<bool>;
#[doc = "Field `CH10GR8_1` writer - Analog switch control"]
pub type CH10GR8_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR1_SPEC, bool, O>;
#[doc = "Field `CH11GR8_2` reader - Analog switch control"]
pub type CH11GR8_2_R = crate::BitReader<bool>;
#[doc = "Field `CH11GR8_2` writer - Analog switch control"]
pub type CH11GR8_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR1_SPEC, bool, O>;
#[doc = "Field `CH12GR8_3` reader - Analog switch control"]
pub type CH12GR8_3_R = crate::BitReader<bool>;
#[doc = "Field `CH12GR8_3` writer - Analog switch control"]
pub type CH12GR8_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR1_SPEC, bool, O>;
#[doc = "Field `CH13GR8_4` reader - Analog switch control"]
pub type CH13GR8_4_R = crate::BitReader<bool>;
#[doc = "Field `CH13GR8_4` writer - Analog switch control"]
pub type CH13GR8_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR1_SPEC, bool, O>;
#[doc = "Field `CH14GR9_1` reader - Analog switch control"]
pub type CH14GR9_1_R = crate::BitReader<bool>;
#[doc = "Field `CH14GR9_1` writer - Analog switch control"]
pub type CH14GR9_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR1_SPEC, bool, O>;
#[doc = "Field `CH15GR9_2` reader - Analog switch control"]
pub type CH15GR9_2_R = crate::BitReader<bool>;
#[doc = "Field `CH15GR9_2` writer - Analog switch control"]
pub type CH15GR9_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR1_SPEC, bool, O>;
#[doc = "Field `CH31GR7_1` reader - Analog switch control"]
pub type CH31GR7_1_R = crate::BitReader<bool>;
#[doc = "Field `CH31GR7_1` writer - Analog switch control"]
pub type CH31GR7_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR1_SPEC, bool, O>;
#[doc = "Field `CH18GR7_1` reader - Analog switch control"]
pub type CH18GR7_1_R = crate::BitReader<bool>;
#[doc = "Field `CH18GR7_1` writer - Analog switch control"]
pub type CH18GR7_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR1_SPEC, bool, O>;
#[doc = "Field `CH19GR7_2` reader - Analog switch control"]
pub type CH19GR7_2_R = crate::BitReader<bool>;
#[doc = "Field `CH19GR7_2` writer - Analog switch control"]
pub type CH19GR7_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR1_SPEC, bool, O>;
#[doc = "Field `CH20GR7_3` reader - Analog switch control"]
pub type CH20GR7_3_R = crate::BitReader<bool>;
#[doc = "Field `CH20GR7_3` writer - Analog switch control"]
pub type CH20GR7_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR1_SPEC, bool, O>;
#[doc = "Field `CH21GR7_4` reader - Analog switch control"]
pub type CH21GR7_4_R = crate::BitReader<bool>;
#[doc = "Field `CH21GR7_4` writer - Analog switch control"]
pub type CH21GR7_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR1_SPEC, bool, O>;
#[doc = "Field `CH22` reader - Analog I/O switch control of channel CH22"]
pub type CH22_R = crate::BitReader<bool>;
#[doc = "Field `CH22` writer - Analog I/O switch control of channel CH22"]
pub type CH22_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR1_SPEC, bool, O>;
#[doc = "Field `CH23` reader - Analog I/O switch control of channel CH23"]
pub type CH23_R = crate::BitReader<bool>;
#[doc = "Field `CH23` writer - Analog I/O switch control of channel CH23"]
pub type CH23_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR1_SPEC, bool, O>;
#[doc = "Field `CH24` reader - Analog I/O switch control of channel CH24"]
pub type CH24_R = crate::BitReader<bool>;
#[doc = "Field `CH24` writer - Analog I/O switch control of channel CH24"]
pub type CH24_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR1_SPEC, bool, O>;
#[doc = "Field `CH25` reader - Analog I/O switch control of channel CH25"]
pub type CH25_R = crate::BitReader<bool>;
#[doc = "Field `CH25` writer - Analog I/O switch control of channel CH25"]
pub type CH25_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR1_SPEC, bool, O>;
#[doc = "Field `VCOMP` reader - ADC analog switch selection for internal node to comparator 1"]
pub type VCOMP_R = crate::BitReader<bool>;
#[doc = "Field `VCOMP` writer - ADC analog switch selection for internal node to comparator 1"]
pub type VCOMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR1_SPEC, bool, O>;
#[doc = "Field `CH27GR11_1` reader - Analog switch control"]
pub type CH27GR11_1_R = crate::BitReader<bool>;
#[doc = "Field `CH27GR11_1` writer - Analog switch control"]
pub type CH27GR11_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR1_SPEC, bool, O>;
#[doc = "Field `CH28GR11_2` reader - Analog switch control"]
pub type CH28GR11_2_R = crate::BitReader<bool>;
#[doc = "Field `CH28GR11_2` writer - Analog switch control"]
pub type CH28GR11_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR1_SPEC, bool, O>;
#[doc = "Field `CH29GR11_3` reader - Analog switch control"]
pub type CH29GR11_3_R = crate::BitReader<bool>;
#[doc = "Field `CH29GR11_3` writer - Analog switch control"]
pub type CH29GR11_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR1_SPEC, bool, O>;
#[doc = "Field `CH30GR11_4` reader - Analog switch control"]
pub type CH30GR11_4_R = crate::BitReader<bool>;
#[doc = "Field `CH30GR11_4` writer - Analog switch control"]
pub type CH30GR11_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR1_SPEC, bool, O>;
#[doc = "Field `SCM` reader - Switch control mode"]
pub type SCM_R = crate::BitReader<bool>;
#[doc = "Field `SCM` writer - Switch control mode"]
pub type SCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Analog switch control"]
    #[inline(always)]
    pub fn ch0gr1_1(&self) -> CH0GR1_1_R {
        CH0GR1_1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog switch control"]
    #[inline(always)]
    pub fn ch1gr1_2(&self) -> CH1GR1_2_R {
        CH1GR1_2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Analog switch control"]
    #[inline(always)]
    pub fn ch2gr1_3(&self) -> CH2GR1_3_R {
        CH2GR1_3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Analog switch control"]
    #[inline(always)]
    pub fn ch3gr1_4(&self) -> CH3GR1_4_R {
        CH3GR1_4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog switch control"]
    #[inline(always)]
    pub fn ch31gr11_5(&self) -> CH31GR11_5_R {
        CH31GR11_5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Comparator 1 analog switch"]
    #[inline(always)]
    pub fn comp1_sw1(&self) -> COMP1_SW1_R {
        COMP1_SW1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Analog switch control"]
    #[inline(always)]
    pub fn ch6gr2_1(&self) -> CH6GR2_1_R {
        CH6GR2_1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog switch control"]
    #[inline(always)]
    pub fn ch7gr2_2(&self) -> CH7GR2_2_R {
        CH7GR2_2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Analog switch control"]
    #[inline(always)]
    pub fn ch8gr3_1(&self) -> CH8GR3_1_R {
        CH8GR3_1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Analog switch control"]
    #[inline(always)]
    pub fn ch9gr3_2(&self) -> CH9GR3_2_R {
        CH9GR3_2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Analog switch control"]
    #[inline(always)]
    pub fn ch10gr8_1(&self) -> CH10GR8_1_R {
        CH10GR8_1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Analog switch control"]
    #[inline(always)]
    pub fn ch11gr8_2(&self) -> CH11GR8_2_R {
        CH11GR8_2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Analog switch control"]
    #[inline(always)]
    pub fn ch12gr8_3(&self) -> CH12GR8_3_R {
        CH12GR8_3_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Analog switch control"]
    #[inline(always)]
    pub fn ch13gr8_4(&self) -> CH13GR8_4_R {
        CH13GR8_4_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Analog switch control"]
    #[inline(always)]
    pub fn ch14gr9_1(&self) -> CH14GR9_1_R {
        CH14GR9_1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Analog switch control"]
    #[inline(always)]
    pub fn ch15gr9_2(&self) -> CH15GR9_2_R {
        CH15GR9_2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Analog switch control"]
    #[inline(always)]
    pub fn ch31gr7_1(&self) -> CH31GR7_1_R {
        CH31GR7_1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Analog switch control"]
    #[inline(always)]
    pub fn ch18gr7_1(&self) -> CH18GR7_1_R {
        CH18GR7_1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Analog switch control"]
    #[inline(always)]
    pub fn ch19gr7_2(&self) -> CH19GR7_2_R {
        CH19GR7_2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Analog switch control"]
    #[inline(always)]
    pub fn ch20gr7_3(&self) -> CH20GR7_3_R {
        CH20GR7_3_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Analog switch control"]
    #[inline(always)]
    pub fn ch21gr7_4(&self) -> CH21GR7_4_R {
        CH21GR7_4_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Analog I/O switch control of channel CH22"]
    #[inline(always)]
    pub fn ch22(&self) -> CH22_R {
        CH22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Analog I/O switch control of channel CH23"]
    #[inline(always)]
    pub fn ch23(&self) -> CH23_R {
        CH23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Analog I/O switch control of channel CH24"]
    #[inline(always)]
    pub fn ch24(&self) -> CH24_R {
        CH24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Analog I/O switch control of channel CH25"]
    #[inline(always)]
    pub fn ch25(&self) -> CH25_R {
        CH25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - ADC analog switch selection for internal node to comparator 1"]
    #[inline(always)]
    pub fn vcomp(&self) -> VCOMP_R {
        VCOMP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Analog switch control"]
    #[inline(always)]
    pub fn ch27gr11_1(&self) -> CH27GR11_1_R {
        CH27GR11_1_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Analog switch control"]
    #[inline(always)]
    pub fn ch28gr11_2(&self) -> CH28GR11_2_R {
        CH28GR11_2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Analog switch control"]
    #[inline(always)]
    pub fn ch29gr11_3(&self) -> CH29GR11_3_R {
        CH29GR11_3_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Analog switch control"]
    #[inline(always)]
    pub fn ch30gr11_4(&self) -> CH30GR11_4_R {
        CH30GR11_4_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Switch control mode"]
    #[inline(always)]
    pub fn scm(&self) -> SCM_R {
        SCM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn ch0gr1_1(&mut self) -> CH0GR1_1_W<0> {
        CH0GR1_1_W::new(self)
    }
    #[doc = "Bit 1 - Analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn ch1gr1_2(&mut self) -> CH1GR1_2_W<1> {
        CH1GR1_2_W::new(self)
    }
    #[doc = "Bit 2 - Analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn ch2gr1_3(&mut self) -> CH2GR1_3_W<2> {
        CH2GR1_3_W::new(self)
    }
    #[doc = "Bit 3 - Analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn ch3gr1_4(&mut self) -> CH3GR1_4_W<3> {
        CH3GR1_4_W::new(self)
    }
    #[doc = "Bit 4 - Analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn ch31gr11_5(&mut self) -> CH31GR11_5_W<4> {
        CH31GR11_5_W::new(self)
    }
    #[doc = "Bit 5 - Comparator 1 analog switch"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_sw1(&mut self) -> COMP1_SW1_W<5> {
        COMP1_SW1_W::new(self)
    }
    #[doc = "Bit 6 - Analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn ch6gr2_1(&mut self) -> CH6GR2_1_W<6> {
        CH6GR2_1_W::new(self)
    }
    #[doc = "Bit 7 - Analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn ch7gr2_2(&mut self) -> CH7GR2_2_W<7> {
        CH7GR2_2_W::new(self)
    }
    #[doc = "Bit 8 - Analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn ch8gr3_1(&mut self) -> CH8GR3_1_W<8> {
        CH8GR3_1_W::new(self)
    }
    #[doc = "Bit 9 - Analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn ch9gr3_2(&mut self) -> CH9GR3_2_W<9> {
        CH9GR3_2_W::new(self)
    }
    #[doc = "Bit 10 - Analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn ch10gr8_1(&mut self) -> CH10GR8_1_W<10> {
        CH10GR8_1_W::new(self)
    }
    #[doc = "Bit 11 - Analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn ch11gr8_2(&mut self) -> CH11GR8_2_W<11> {
        CH11GR8_2_W::new(self)
    }
    #[doc = "Bit 12 - Analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn ch12gr8_3(&mut self) -> CH12GR8_3_W<12> {
        CH12GR8_3_W::new(self)
    }
    #[doc = "Bit 13 - Analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn ch13gr8_4(&mut self) -> CH13GR8_4_W<13> {
        CH13GR8_4_W::new(self)
    }
    #[doc = "Bit 14 - Analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn ch14gr9_1(&mut self) -> CH14GR9_1_W<14> {
        CH14GR9_1_W::new(self)
    }
    #[doc = "Bit 15 - Analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn ch15gr9_2(&mut self) -> CH15GR9_2_W<15> {
        CH15GR9_2_W::new(self)
    }
    #[doc = "Bit 16 - Analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn ch31gr7_1(&mut self) -> CH31GR7_1_W<16> {
        CH31GR7_1_W::new(self)
    }
    #[doc = "Bit 18 - Analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn ch18gr7_1(&mut self) -> CH18GR7_1_W<18> {
        CH18GR7_1_W::new(self)
    }
    #[doc = "Bit 19 - Analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn ch19gr7_2(&mut self) -> CH19GR7_2_W<19> {
        CH19GR7_2_W::new(self)
    }
    #[doc = "Bit 20 - Analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn ch20gr7_3(&mut self) -> CH20GR7_3_W<20> {
        CH20GR7_3_W::new(self)
    }
    #[doc = "Bit 21 - Analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn ch21gr7_4(&mut self) -> CH21GR7_4_W<21> {
        CH21GR7_4_W::new(self)
    }
    #[doc = "Bit 22 - Analog I/O switch control of channel CH22"]
    #[inline(always)]
    #[must_use]
    pub fn ch22(&mut self) -> CH22_W<22> {
        CH22_W::new(self)
    }
    #[doc = "Bit 23 - Analog I/O switch control of channel CH23"]
    #[inline(always)]
    #[must_use]
    pub fn ch23(&mut self) -> CH23_W<23> {
        CH23_W::new(self)
    }
    #[doc = "Bit 24 - Analog I/O switch control of channel CH24"]
    #[inline(always)]
    #[must_use]
    pub fn ch24(&mut self) -> CH24_W<24> {
        CH24_W::new(self)
    }
    #[doc = "Bit 25 - Analog I/O switch control of channel CH25"]
    #[inline(always)]
    #[must_use]
    pub fn ch25(&mut self) -> CH25_W<25> {
        CH25_W::new(self)
    }
    #[doc = "Bit 26 - ADC analog switch selection for internal node to comparator 1"]
    #[inline(always)]
    #[must_use]
    pub fn vcomp(&mut self) -> VCOMP_W<26> {
        VCOMP_W::new(self)
    }
    #[doc = "Bit 27 - Analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn ch27gr11_1(&mut self) -> CH27GR11_1_W<27> {
        CH27GR11_1_W::new(self)
    }
    #[doc = "Bit 28 - Analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn ch28gr11_2(&mut self) -> CH28GR11_2_W<28> {
        CH28GR11_2_W::new(self)
    }
    #[doc = "Bit 29 - Analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn ch29gr11_3(&mut self) -> CH29GR11_3_W<29> {
        CH29GR11_3_W::new(self)
    }
    #[doc = "Bit 30 - Analog switch control"]
    #[inline(always)]
    #[must_use]
    pub fn ch30gr11_4(&mut self) -> CH30GR11_4_W<30> {
        CH30GR11_4_W::new(self)
    }
    #[doc = "Bit 31 - Switch control mode"]
    #[inline(always)]
    #[must_use]
    pub fn scm(&mut self) -> SCM_W<31> {
        SCM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RI analog switches control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ascr1](index.html) module"]
pub struct ASCR1_SPEC;
impl crate::RegisterSpec for ASCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ascr1::R](R) reader structure"]
impl crate::Readable for ASCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ascr1::W](W) writer structure"]
impl crate::Writable for ASCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ASCR1 to value 0"]
impl crate::Resettable for ASCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
