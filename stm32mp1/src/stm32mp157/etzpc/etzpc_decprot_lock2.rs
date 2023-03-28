#[doc = "Register `ETZPC_DECPROT_LOCK2` reader"]
pub struct R(crate::R<ETZPC_DECPROT_LOCK2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETZPC_DECPROT_LOCK2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETZPC_DECPROT_LOCK2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETZPC_DECPROT_LOCK2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETZPC_DECPROT_LOCK2` writer"]
pub struct W(crate::W<ETZPC_DECPROT_LOCK2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETZPC_DECPROT_LOCK2_SPEC>;
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
impl From<crate::W<ETZPC_DECPROT_LOCK2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETZPC_DECPROT_LOCK2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK0` reader - LOCK0"]
pub type LOCK0_R = crate::BitReader<bool>;
#[doc = "Field `LOCK0` writer - LOCK0"]
pub type LOCK0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
#[doc = "Field `LOCK1` reader - LOCK1"]
pub type LOCK1_R = crate::BitReader<bool>;
#[doc = "Field `LOCK1` writer - LOCK1"]
pub type LOCK1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
#[doc = "Field `LOCK2` reader - LOCK2"]
pub type LOCK2_R = crate::BitReader<bool>;
#[doc = "Field `LOCK2` writer - LOCK2"]
pub type LOCK2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
#[doc = "Field `LOCK3` reader - LOCK3"]
pub type LOCK3_R = crate::BitReader<bool>;
#[doc = "Field `LOCK3` writer - LOCK3"]
pub type LOCK3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
#[doc = "Field `LOCK4` reader - LOCK4"]
pub type LOCK4_R = crate::BitReader<bool>;
#[doc = "Field `LOCK4` writer - LOCK4"]
pub type LOCK4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
#[doc = "Field `LOCK5` reader - LOCK5"]
pub type LOCK5_R = crate::BitReader<bool>;
#[doc = "Field `LOCK5` writer - LOCK5"]
pub type LOCK5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
#[doc = "Field `LOCK6` reader - LOCK6"]
pub type LOCK6_R = crate::BitReader<bool>;
#[doc = "Field `LOCK6` writer - LOCK6"]
pub type LOCK6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
#[doc = "Field `LOCK7` reader - LOCK7"]
pub type LOCK7_R = crate::BitReader<bool>;
#[doc = "Field `LOCK7` writer - LOCK7"]
pub type LOCK7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
#[doc = "Field `LOCK8` reader - LOCK8"]
pub type LOCK8_R = crate::BitReader<bool>;
#[doc = "Field `LOCK8` writer - LOCK8"]
pub type LOCK8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
#[doc = "Field `LOCK9` reader - LOCK9"]
pub type LOCK9_R = crate::BitReader<bool>;
#[doc = "Field `LOCK9` writer - LOCK9"]
pub type LOCK9_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
#[doc = "Field `LOCK10` reader - LOCK10"]
pub type LOCK10_R = crate::BitReader<bool>;
#[doc = "Field `LOCK10` writer - LOCK10"]
pub type LOCK10_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
#[doc = "Field `LOCK11` reader - LOCK11"]
pub type LOCK11_R = crate::BitReader<bool>;
#[doc = "Field `LOCK11` writer - LOCK11"]
pub type LOCK11_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
#[doc = "Field `LOCK12` reader - LOCK12"]
pub type LOCK12_R = crate::BitReader<bool>;
#[doc = "Field `LOCK12` writer - LOCK12"]
pub type LOCK12_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
#[doc = "Field `LOCK13` reader - LOCK13"]
pub type LOCK13_R = crate::BitReader<bool>;
#[doc = "Field `LOCK13` writer - LOCK13"]
pub type LOCK13_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
#[doc = "Field `LOCK14` reader - LOCK14"]
pub type LOCK14_R = crate::BitReader<bool>;
#[doc = "Field `LOCK14` writer - LOCK14"]
pub type LOCK14_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
#[doc = "Field `LOCK15` reader - LOCK15"]
pub type LOCK15_R = crate::BitReader<bool>;
#[doc = "Field `LOCK15` writer - LOCK15"]
pub type LOCK15_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
#[doc = "Field `LOCK16` reader - LOCK16"]
pub type LOCK16_R = crate::BitReader<bool>;
#[doc = "Field `LOCK16` writer - LOCK16"]
pub type LOCK16_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
#[doc = "Field `LOCK17` reader - LOCK17"]
pub type LOCK17_R = crate::BitReader<bool>;
#[doc = "Field `LOCK17` writer - LOCK17"]
pub type LOCK17_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
#[doc = "Field `LOCK18` reader - LOCK18"]
pub type LOCK18_R = crate::BitReader<bool>;
#[doc = "Field `LOCK18` writer - LOCK18"]
pub type LOCK18_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
#[doc = "Field `LOCK19` reader - LOCK19"]
pub type LOCK19_R = crate::BitReader<bool>;
#[doc = "Field `LOCK19` writer - LOCK19"]
pub type LOCK19_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
#[doc = "Field `LOCK20` reader - LOCK20"]
pub type LOCK20_R = crate::BitReader<bool>;
#[doc = "Field `LOCK20` writer - LOCK20"]
pub type LOCK20_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
#[doc = "Field `LOCK21` reader - LOCK21"]
pub type LOCK21_R = crate::BitReader<bool>;
#[doc = "Field `LOCK21` writer - LOCK21"]
pub type LOCK21_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
#[doc = "Field `LOCK22` reader - LOCK22"]
pub type LOCK22_R = crate::BitReader<bool>;
#[doc = "Field `LOCK22` writer - LOCK22"]
pub type LOCK22_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
#[doc = "Field `LOCK23` reader - LOCK23"]
pub type LOCK23_R = crate::BitReader<bool>;
#[doc = "Field `LOCK23` writer - LOCK23"]
pub type LOCK23_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
#[doc = "Field `LOCK24` reader - LOCK24"]
pub type LOCK24_R = crate::BitReader<bool>;
#[doc = "Field `LOCK24` writer - LOCK24"]
pub type LOCK24_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
#[doc = "Field `LOCK25` reader - LOCK25"]
pub type LOCK25_R = crate::BitReader<bool>;
#[doc = "Field `LOCK25` writer - LOCK25"]
pub type LOCK25_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
#[doc = "Field `LOCK26` reader - LOCK26"]
pub type LOCK26_R = crate::BitReader<bool>;
#[doc = "Field `LOCK26` writer - LOCK26"]
pub type LOCK26_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
#[doc = "Field `LOCK27` reader - LOCK27"]
pub type LOCK27_R = crate::BitReader<bool>;
#[doc = "Field `LOCK27` writer - LOCK27"]
pub type LOCK27_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
#[doc = "Field `LOCK28` reader - LOCK28"]
pub type LOCK28_R = crate::BitReader<bool>;
#[doc = "Field `LOCK28` writer - LOCK28"]
pub type LOCK28_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
#[doc = "Field `LOCK29` reader - LOCK29"]
pub type LOCK29_R = crate::BitReader<bool>;
#[doc = "Field `LOCK29` writer - LOCK29"]
pub type LOCK29_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
#[doc = "Field `LOCK30` reader - LOCK30"]
pub type LOCK30_R = crate::BitReader<bool>;
#[doc = "Field `LOCK30` writer - LOCK30"]
pub type LOCK30_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
#[doc = "Field `LOCK31` reader - LOCK31"]
pub type LOCK31_R = crate::BitReader<bool>;
#[doc = "Field `LOCK31` writer - LOCK31"]
pub type LOCK31_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_DECPROT_LOCK2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LOCK0"]
    #[inline(always)]
    pub fn lock0(&self) -> LOCK0_R {
        LOCK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LOCK1"]
    #[inline(always)]
    pub fn lock1(&self) -> LOCK1_R {
        LOCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LOCK2"]
    #[inline(always)]
    pub fn lock2(&self) -> LOCK2_R {
        LOCK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LOCK3"]
    #[inline(always)]
    pub fn lock3(&self) -> LOCK3_R {
        LOCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LOCK4"]
    #[inline(always)]
    pub fn lock4(&self) -> LOCK4_R {
        LOCK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LOCK5"]
    #[inline(always)]
    pub fn lock5(&self) -> LOCK5_R {
        LOCK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LOCK6"]
    #[inline(always)]
    pub fn lock6(&self) -> LOCK6_R {
        LOCK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LOCK7"]
    #[inline(always)]
    pub fn lock7(&self) -> LOCK7_R {
        LOCK7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LOCK8"]
    #[inline(always)]
    pub fn lock8(&self) -> LOCK8_R {
        LOCK8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LOCK9"]
    #[inline(always)]
    pub fn lock9(&self) -> LOCK9_R {
        LOCK9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LOCK10"]
    #[inline(always)]
    pub fn lock10(&self) -> LOCK10_R {
        LOCK10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LOCK11"]
    #[inline(always)]
    pub fn lock11(&self) -> LOCK11_R {
        LOCK11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LOCK12"]
    #[inline(always)]
    pub fn lock12(&self) -> LOCK12_R {
        LOCK12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LOCK13"]
    #[inline(always)]
    pub fn lock13(&self) -> LOCK13_R {
        LOCK13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LOCK14"]
    #[inline(always)]
    pub fn lock14(&self) -> LOCK14_R {
        LOCK14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LOCK15"]
    #[inline(always)]
    pub fn lock15(&self) -> LOCK15_R {
        LOCK15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - LOCK16"]
    #[inline(always)]
    pub fn lock16(&self) -> LOCK16_R {
        LOCK16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - LOCK17"]
    #[inline(always)]
    pub fn lock17(&self) -> LOCK17_R {
        LOCK17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - LOCK18"]
    #[inline(always)]
    pub fn lock18(&self) -> LOCK18_R {
        LOCK18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - LOCK19"]
    #[inline(always)]
    pub fn lock19(&self) -> LOCK19_R {
        LOCK19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LOCK20"]
    #[inline(always)]
    pub fn lock20(&self) -> LOCK20_R {
        LOCK20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - LOCK21"]
    #[inline(always)]
    pub fn lock21(&self) -> LOCK21_R {
        LOCK21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - LOCK22"]
    #[inline(always)]
    pub fn lock22(&self) -> LOCK22_R {
        LOCK22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - LOCK23"]
    #[inline(always)]
    pub fn lock23(&self) -> LOCK23_R {
        LOCK23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - LOCK24"]
    #[inline(always)]
    pub fn lock24(&self) -> LOCK24_R {
        LOCK24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - LOCK25"]
    #[inline(always)]
    pub fn lock25(&self) -> LOCK25_R {
        LOCK25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LOCK26"]
    #[inline(always)]
    pub fn lock26(&self) -> LOCK26_R {
        LOCK26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LOCK27"]
    #[inline(always)]
    pub fn lock27(&self) -> LOCK27_R {
        LOCK27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - LOCK28"]
    #[inline(always)]
    pub fn lock28(&self) -> LOCK28_R {
        LOCK28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - LOCK29"]
    #[inline(always)]
    pub fn lock29(&self) -> LOCK29_R {
        LOCK29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - LOCK30"]
    #[inline(always)]
    pub fn lock30(&self) -> LOCK30_R {
        LOCK30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - LOCK31"]
    #[inline(always)]
    pub fn lock31(&self) -> LOCK31_R {
        LOCK31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LOCK0"]
    #[inline(always)]
    #[must_use]
    pub fn lock0(&mut self) -> LOCK0_W<0> {
        LOCK0_W::new(self)
    }
    #[doc = "Bit 1 - LOCK1"]
    #[inline(always)]
    #[must_use]
    pub fn lock1(&mut self) -> LOCK1_W<1> {
        LOCK1_W::new(self)
    }
    #[doc = "Bit 2 - LOCK2"]
    #[inline(always)]
    #[must_use]
    pub fn lock2(&mut self) -> LOCK2_W<2> {
        LOCK2_W::new(self)
    }
    #[doc = "Bit 3 - LOCK3"]
    #[inline(always)]
    #[must_use]
    pub fn lock3(&mut self) -> LOCK3_W<3> {
        LOCK3_W::new(self)
    }
    #[doc = "Bit 4 - LOCK4"]
    #[inline(always)]
    #[must_use]
    pub fn lock4(&mut self) -> LOCK4_W<4> {
        LOCK4_W::new(self)
    }
    #[doc = "Bit 5 - LOCK5"]
    #[inline(always)]
    #[must_use]
    pub fn lock5(&mut self) -> LOCK5_W<5> {
        LOCK5_W::new(self)
    }
    #[doc = "Bit 6 - LOCK6"]
    #[inline(always)]
    #[must_use]
    pub fn lock6(&mut self) -> LOCK6_W<6> {
        LOCK6_W::new(self)
    }
    #[doc = "Bit 7 - LOCK7"]
    #[inline(always)]
    #[must_use]
    pub fn lock7(&mut self) -> LOCK7_W<7> {
        LOCK7_W::new(self)
    }
    #[doc = "Bit 8 - LOCK8"]
    #[inline(always)]
    #[must_use]
    pub fn lock8(&mut self) -> LOCK8_W<8> {
        LOCK8_W::new(self)
    }
    #[doc = "Bit 9 - LOCK9"]
    #[inline(always)]
    #[must_use]
    pub fn lock9(&mut self) -> LOCK9_W<9> {
        LOCK9_W::new(self)
    }
    #[doc = "Bit 10 - LOCK10"]
    #[inline(always)]
    #[must_use]
    pub fn lock10(&mut self) -> LOCK10_W<10> {
        LOCK10_W::new(self)
    }
    #[doc = "Bit 11 - LOCK11"]
    #[inline(always)]
    #[must_use]
    pub fn lock11(&mut self) -> LOCK11_W<11> {
        LOCK11_W::new(self)
    }
    #[doc = "Bit 12 - LOCK12"]
    #[inline(always)]
    #[must_use]
    pub fn lock12(&mut self) -> LOCK12_W<12> {
        LOCK12_W::new(self)
    }
    #[doc = "Bit 13 - LOCK13"]
    #[inline(always)]
    #[must_use]
    pub fn lock13(&mut self) -> LOCK13_W<13> {
        LOCK13_W::new(self)
    }
    #[doc = "Bit 14 - LOCK14"]
    #[inline(always)]
    #[must_use]
    pub fn lock14(&mut self) -> LOCK14_W<14> {
        LOCK14_W::new(self)
    }
    #[doc = "Bit 15 - LOCK15"]
    #[inline(always)]
    #[must_use]
    pub fn lock15(&mut self) -> LOCK15_W<15> {
        LOCK15_W::new(self)
    }
    #[doc = "Bit 16 - LOCK16"]
    #[inline(always)]
    #[must_use]
    pub fn lock16(&mut self) -> LOCK16_W<16> {
        LOCK16_W::new(self)
    }
    #[doc = "Bit 17 - LOCK17"]
    #[inline(always)]
    #[must_use]
    pub fn lock17(&mut self) -> LOCK17_W<17> {
        LOCK17_W::new(self)
    }
    #[doc = "Bit 18 - LOCK18"]
    #[inline(always)]
    #[must_use]
    pub fn lock18(&mut self) -> LOCK18_W<18> {
        LOCK18_W::new(self)
    }
    #[doc = "Bit 19 - LOCK19"]
    #[inline(always)]
    #[must_use]
    pub fn lock19(&mut self) -> LOCK19_W<19> {
        LOCK19_W::new(self)
    }
    #[doc = "Bit 20 - LOCK20"]
    #[inline(always)]
    #[must_use]
    pub fn lock20(&mut self) -> LOCK20_W<20> {
        LOCK20_W::new(self)
    }
    #[doc = "Bit 21 - LOCK21"]
    #[inline(always)]
    #[must_use]
    pub fn lock21(&mut self) -> LOCK21_W<21> {
        LOCK21_W::new(self)
    }
    #[doc = "Bit 22 - LOCK22"]
    #[inline(always)]
    #[must_use]
    pub fn lock22(&mut self) -> LOCK22_W<22> {
        LOCK22_W::new(self)
    }
    #[doc = "Bit 23 - LOCK23"]
    #[inline(always)]
    #[must_use]
    pub fn lock23(&mut self) -> LOCK23_W<23> {
        LOCK23_W::new(self)
    }
    #[doc = "Bit 24 - LOCK24"]
    #[inline(always)]
    #[must_use]
    pub fn lock24(&mut self) -> LOCK24_W<24> {
        LOCK24_W::new(self)
    }
    #[doc = "Bit 25 - LOCK25"]
    #[inline(always)]
    #[must_use]
    pub fn lock25(&mut self) -> LOCK25_W<25> {
        LOCK25_W::new(self)
    }
    #[doc = "Bit 26 - LOCK26"]
    #[inline(always)]
    #[must_use]
    pub fn lock26(&mut self) -> LOCK26_W<26> {
        LOCK26_W::new(self)
    }
    #[doc = "Bit 27 - LOCK27"]
    #[inline(always)]
    #[must_use]
    pub fn lock27(&mut self) -> LOCK27_W<27> {
        LOCK27_W::new(self)
    }
    #[doc = "Bit 28 - LOCK28"]
    #[inline(always)]
    #[must_use]
    pub fn lock28(&mut self) -> LOCK28_W<28> {
        LOCK28_W::new(self)
    }
    #[doc = "Bit 29 - LOCK29"]
    #[inline(always)]
    #[must_use]
    pub fn lock29(&mut self) -> LOCK29_W<29> {
        LOCK29_W::new(self)
    }
    #[doc = "Bit 30 - LOCK30"]
    #[inline(always)]
    #[must_use]
    pub fn lock30(&mut self) -> LOCK30_W<30> {
        LOCK30_W::new(self)
    }
    #[doc = "Bit 31 - LOCK31"]
    #[inline(always)]
    #[must_use]
    pub fn lock31(&mut self) -> LOCK31_W<31> {
        LOCK31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETZPC decprot lock 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_decprot_lock2](index.html) module"]
pub struct ETZPC_DECPROT_LOCK2_SPEC;
impl crate::RegisterSpec for ETZPC_DECPROT_LOCK2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etzpc_decprot_lock2::R](R) reader structure"]
impl crate::Readable for ETZPC_DECPROT_LOCK2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etzpc_decprot_lock2::W](W) writer structure"]
impl crate::Writable for ETZPC_DECPROT_LOCK2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETZPC_DECPROT_LOCK2 to value 0"]
impl crate::Resettable for ETZPC_DECPROT_LOCK2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
