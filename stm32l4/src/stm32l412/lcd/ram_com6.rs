#[doc = "Register `RAM_COM6` reader"]
pub struct R(crate::R<RAM_COM6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM_COM6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM_COM6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM_COM6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAM_COM6` writer"]
pub struct W(crate::W<RAM_COM6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAM_COM6_SPEC>;
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
impl From<crate::W<RAM_COM6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAM_COM6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `S00` reader - S00"]
pub type S00_R = crate::BitReader<bool>;
#[doc = "Field `S00` writer - S00"]
pub type S00_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
#[doc = "Field `S01` reader - S01"]
pub type S01_R = crate::BitReader<bool>;
#[doc = "Field `S01` writer - S01"]
pub type S01_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
#[doc = "Field `S02` reader - S02"]
pub type S02_R = crate::BitReader<bool>;
#[doc = "Field `S02` writer - S02"]
pub type S02_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
#[doc = "Field `S03` reader - S03"]
pub type S03_R = crate::BitReader<bool>;
#[doc = "Field `S03` writer - S03"]
pub type S03_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
#[doc = "Field `S04` reader - S04"]
pub type S04_R = crate::BitReader<bool>;
#[doc = "Field `S04` writer - S04"]
pub type S04_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
#[doc = "Field `S05` reader - S05"]
pub type S05_R = crate::BitReader<bool>;
#[doc = "Field `S05` writer - S05"]
pub type S05_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
#[doc = "Field `S06` reader - S06"]
pub type S06_R = crate::BitReader<bool>;
#[doc = "Field `S06` writer - S06"]
pub type S06_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
#[doc = "Field `S07` reader - S07"]
pub type S07_R = crate::BitReader<bool>;
#[doc = "Field `S07` writer - S07"]
pub type S07_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
#[doc = "Field `S08` reader - S08"]
pub type S08_R = crate::BitReader<bool>;
#[doc = "Field `S08` writer - S08"]
pub type S08_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
#[doc = "Field `S09` reader - S09"]
pub type S09_R = crate::BitReader<bool>;
#[doc = "Field `S09` writer - S09"]
pub type S09_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
#[doc = "Field `S10` reader - S10"]
pub type S10_R = crate::BitReader<bool>;
#[doc = "Field `S10` writer - S10"]
pub type S10_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
#[doc = "Field `S11` reader - S11"]
pub type S11_R = crate::BitReader<bool>;
#[doc = "Field `S11` writer - S11"]
pub type S11_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
#[doc = "Field `S12` reader - S12"]
pub type S12_R = crate::BitReader<bool>;
#[doc = "Field `S12` writer - S12"]
pub type S12_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
#[doc = "Field `S13` reader - S13"]
pub type S13_R = crate::BitReader<bool>;
#[doc = "Field `S13` writer - S13"]
pub type S13_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
#[doc = "Field `S14` reader - S14"]
pub type S14_R = crate::BitReader<bool>;
#[doc = "Field `S14` writer - S14"]
pub type S14_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
#[doc = "Field `S15` reader - S15"]
pub type S15_R = crate::BitReader<bool>;
#[doc = "Field `S15` writer - S15"]
pub type S15_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
#[doc = "Field `S16` reader - S16"]
pub type S16_R = crate::BitReader<bool>;
#[doc = "Field `S16` writer - S16"]
pub type S16_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
#[doc = "Field `S17` reader - S17"]
pub type S17_R = crate::BitReader<bool>;
#[doc = "Field `S17` writer - S17"]
pub type S17_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
#[doc = "Field `S18` reader - S18"]
pub type S18_R = crate::BitReader<bool>;
#[doc = "Field `S18` writer - S18"]
pub type S18_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
#[doc = "Field `S19` reader - S19"]
pub type S19_R = crate::BitReader<bool>;
#[doc = "Field `S19` writer - S19"]
pub type S19_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
#[doc = "Field `S20` reader - S20"]
pub type S20_R = crate::BitReader<bool>;
#[doc = "Field `S20` writer - S20"]
pub type S20_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
#[doc = "Field `S21` reader - S21"]
pub type S21_R = crate::BitReader<bool>;
#[doc = "Field `S21` writer - S21"]
pub type S21_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
#[doc = "Field `S22` reader - S22"]
pub type S22_R = crate::BitReader<bool>;
#[doc = "Field `S22` writer - S22"]
pub type S22_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
#[doc = "Field `S23` reader - S23"]
pub type S23_R = crate::BitReader<bool>;
#[doc = "Field `S23` writer - S23"]
pub type S23_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
#[doc = "Field `S24` reader - S24"]
pub type S24_R = crate::BitReader<bool>;
#[doc = "Field `S24` writer - S24"]
pub type S24_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
#[doc = "Field `S25` reader - S25"]
pub type S25_R = crate::BitReader<bool>;
#[doc = "Field `S25` writer - S25"]
pub type S25_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
#[doc = "Field `S26` reader - S26"]
pub type S26_R = crate::BitReader<bool>;
#[doc = "Field `S26` writer - S26"]
pub type S26_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
#[doc = "Field `S27` reader - S27"]
pub type S27_R = crate::BitReader<bool>;
#[doc = "Field `S27` writer - S27"]
pub type S27_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
#[doc = "Field `S28` reader - S28"]
pub type S28_R = crate::BitReader<bool>;
#[doc = "Field `S28` writer - S28"]
pub type S28_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
#[doc = "Field `S29` reader - S29"]
pub type S29_R = crate::BitReader<bool>;
#[doc = "Field `S29` writer - S29"]
pub type S29_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
#[doc = "Field `S30` reader - S30"]
pub type S30_R = crate::BitReader<bool>;
#[doc = "Field `S30` writer - S30"]
pub type S30_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
#[doc = "Field `S31` reader - S31"]
pub type S31_R = crate::BitReader<bool>;
#[doc = "Field `S31` writer - S31"]
pub type S31_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM_COM6_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - S00"]
    #[inline(always)]
    pub fn s00(&self) -> S00_R {
        S00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - S01"]
    #[inline(always)]
    pub fn s01(&self) -> S01_R {
        S01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - S02"]
    #[inline(always)]
    pub fn s02(&self) -> S02_R {
        S02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - S03"]
    #[inline(always)]
    pub fn s03(&self) -> S03_R {
        S03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - S04"]
    #[inline(always)]
    pub fn s04(&self) -> S04_R {
        S04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - S05"]
    #[inline(always)]
    pub fn s05(&self) -> S05_R {
        S05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - S06"]
    #[inline(always)]
    pub fn s06(&self) -> S06_R {
        S06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - S07"]
    #[inline(always)]
    pub fn s07(&self) -> S07_R {
        S07_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - S08"]
    #[inline(always)]
    pub fn s08(&self) -> S08_R {
        S08_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - S09"]
    #[inline(always)]
    pub fn s09(&self) -> S09_R {
        S09_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - S10"]
    #[inline(always)]
    pub fn s10(&self) -> S10_R {
        S10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - S11"]
    #[inline(always)]
    pub fn s11(&self) -> S11_R {
        S11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - S12"]
    #[inline(always)]
    pub fn s12(&self) -> S12_R {
        S12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - S13"]
    #[inline(always)]
    pub fn s13(&self) -> S13_R {
        S13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - S14"]
    #[inline(always)]
    pub fn s14(&self) -> S14_R {
        S14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - S15"]
    #[inline(always)]
    pub fn s15(&self) -> S15_R {
        S15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - S16"]
    #[inline(always)]
    pub fn s16(&self) -> S16_R {
        S16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - S17"]
    #[inline(always)]
    pub fn s17(&self) -> S17_R {
        S17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - S18"]
    #[inline(always)]
    pub fn s18(&self) -> S18_R {
        S18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - S19"]
    #[inline(always)]
    pub fn s19(&self) -> S19_R {
        S19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - S20"]
    #[inline(always)]
    pub fn s20(&self) -> S20_R {
        S20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - S21"]
    #[inline(always)]
    pub fn s21(&self) -> S21_R {
        S21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - S22"]
    #[inline(always)]
    pub fn s22(&self) -> S22_R {
        S22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - S23"]
    #[inline(always)]
    pub fn s23(&self) -> S23_R {
        S23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - S24"]
    #[inline(always)]
    pub fn s24(&self) -> S24_R {
        S24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - S25"]
    #[inline(always)]
    pub fn s25(&self) -> S25_R {
        S25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - S26"]
    #[inline(always)]
    pub fn s26(&self) -> S26_R {
        S26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - S27"]
    #[inline(always)]
    pub fn s27(&self) -> S27_R {
        S27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - S28"]
    #[inline(always)]
    pub fn s28(&self) -> S28_R {
        S28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - S29"]
    #[inline(always)]
    pub fn s29(&self) -> S29_R {
        S29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - S30"]
    #[inline(always)]
    pub fn s30(&self) -> S30_R {
        S30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - S31"]
    #[inline(always)]
    pub fn s31(&self) -> S31_R {
        S31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - S00"]
    #[inline(always)]
    #[must_use]
    pub fn s00(&mut self) -> S00_W<0> {
        S00_W::new(self)
    }
    #[doc = "Bit 1 - S01"]
    #[inline(always)]
    #[must_use]
    pub fn s01(&mut self) -> S01_W<1> {
        S01_W::new(self)
    }
    #[doc = "Bit 2 - S02"]
    #[inline(always)]
    #[must_use]
    pub fn s02(&mut self) -> S02_W<2> {
        S02_W::new(self)
    }
    #[doc = "Bit 3 - S03"]
    #[inline(always)]
    #[must_use]
    pub fn s03(&mut self) -> S03_W<3> {
        S03_W::new(self)
    }
    #[doc = "Bit 4 - S04"]
    #[inline(always)]
    #[must_use]
    pub fn s04(&mut self) -> S04_W<4> {
        S04_W::new(self)
    }
    #[doc = "Bit 5 - S05"]
    #[inline(always)]
    #[must_use]
    pub fn s05(&mut self) -> S05_W<5> {
        S05_W::new(self)
    }
    #[doc = "Bit 6 - S06"]
    #[inline(always)]
    #[must_use]
    pub fn s06(&mut self) -> S06_W<6> {
        S06_W::new(self)
    }
    #[doc = "Bit 7 - S07"]
    #[inline(always)]
    #[must_use]
    pub fn s07(&mut self) -> S07_W<7> {
        S07_W::new(self)
    }
    #[doc = "Bit 8 - S08"]
    #[inline(always)]
    #[must_use]
    pub fn s08(&mut self) -> S08_W<8> {
        S08_W::new(self)
    }
    #[doc = "Bit 9 - S09"]
    #[inline(always)]
    #[must_use]
    pub fn s09(&mut self) -> S09_W<9> {
        S09_W::new(self)
    }
    #[doc = "Bit 10 - S10"]
    #[inline(always)]
    #[must_use]
    pub fn s10(&mut self) -> S10_W<10> {
        S10_W::new(self)
    }
    #[doc = "Bit 11 - S11"]
    #[inline(always)]
    #[must_use]
    pub fn s11(&mut self) -> S11_W<11> {
        S11_W::new(self)
    }
    #[doc = "Bit 12 - S12"]
    #[inline(always)]
    #[must_use]
    pub fn s12(&mut self) -> S12_W<12> {
        S12_W::new(self)
    }
    #[doc = "Bit 13 - S13"]
    #[inline(always)]
    #[must_use]
    pub fn s13(&mut self) -> S13_W<13> {
        S13_W::new(self)
    }
    #[doc = "Bit 14 - S14"]
    #[inline(always)]
    #[must_use]
    pub fn s14(&mut self) -> S14_W<14> {
        S14_W::new(self)
    }
    #[doc = "Bit 15 - S15"]
    #[inline(always)]
    #[must_use]
    pub fn s15(&mut self) -> S15_W<15> {
        S15_W::new(self)
    }
    #[doc = "Bit 16 - S16"]
    #[inline(always)]
    #[must_use]
    pub fn s16(&mut self) -> S16_W<16> {
        S16_W::new(self)
    }
    #[doc = "Bit 17 - S17"]
    #[inline(always)]
    #[must_use]
    pub fn s17(&mut self) -> S17_W<17> {
        S17_W::new(self)
    }
    #[doc = "Bit 18 - S18"]
    #[inline(always)]
    #[must_use]
    pub fn s18(&mut self) -> S18_W<18> {
        S18_W::new(self)
    }
    #[doc = "Bit 19 - S19"]
    #[inline(always)]
    #[must_use]
    pub fn s19(&mut self) -> S19_W<19> {
        S19_W::new(self)
    }
    #[doc = "Bit 20 - S20"]
    #[inline(always)]
    #[must_use]
    pub fn s20(&mut self) -> S20_W<20> {
        S20_W::new(self)
    }
    #[doc = "Bit 21 - S21"]
    #[inline(always)]
    #[must_use]
    pub fn s21(&mut self) -> S21_W<21> {
        S21_W::new(self)
    }
    #[doc = "Bit 22 - S22"]
    #[inline(always)]
    #[must_use]
    pub fn s22(&mut self) -> S22_W<22> {
        S22_W::new(self)
    }
    #[doc = "Bit 23 - S23"]
    #[inline(always)]
    #[must_use]
    pub fn s23(&mut self) -> S23_W<23> {
        S23_W::new(self)
    }
    #[doc = "Bit 24 - S24"]
    #[inline(always)]
    #[must_use]
    pub fn s24(&mut self) -> S24_W<24> {
        S24_W::new(self)
    }
    #[doc = "Bit 25 - S25"]
    #[inline(always)]
    #[must_use]
    pub fn s25(&mut self) -> S25_W<25> {
        S25_W::new(self)
    }
    #[doc = "Bit 26 - S26"]
    #[inline(always)]
    #[must_use]
    pub fn s26(&mut self) -> S26_W<26> {
        S26_W::new(self)
    }
    #[doc = "Bit 27 - S27"]
    #[inline(always)]
    #[must_use]
    pub fn s27(&mut self) -> S27_W<27> {
        S27_W::new(self)
    }
    #[doc = "Bit 28 - S28"]
    #[inline(always)]
    #[must_use]
    pub fn s28(&mut self) -> S28_W<28> {
        S28_W::new(self)
    }
    #[doc = "Bit 29 - S29"]
    #[inline(always)]
    #[must_use]
    pub fn s29(&mut self) -> S29_W<29> {
        S29_W::new(self)
    }
    #[doc = "Bit 30 - S30"]
    #[inline(always)]
    #[must_use]
    pub fn s30(&mut self) -> S30_W<30> {
        S30_W::new(self)
    }
    #[doc = "Bit 31 - S31"]
    #[inline(always)]
    #[must_use]
    pub fn s31(&mut self) -> S31_W<31> {
        S31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "display memory\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram_com6](index.html) module"]
pub struct RAM_COM6_SPEC;
impl crate::RegisterSpec for RAM_COM6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ram_com6::R](R) reader structure"]
impl crate::Readable for RAM_COM6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ram_com6::W](W) writer structure"]
impl crate::Writable for RAM_COM6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RAM_COM6 to value 0"]
impl crate::Resettable for RAM_COM6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
