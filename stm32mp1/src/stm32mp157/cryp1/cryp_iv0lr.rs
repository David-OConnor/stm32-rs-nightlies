#[doc = "Register `CRYP_IV0LR` reader"]
pub struct R(crate::R<CRYP_IV0LR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYP_IV0LR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYP_IV0LR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYP_IV0LR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRYP_IV0LR` writer"]
pub struct W(crate::W<CRYP_IV0LR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYP_IV0LR_SPEC>;
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
impl From<crate::W<CRYP_IV0LR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYP_IV0LR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IV31` reader - IV31"]
pub type IV31_R = crate::BitReader<bool>;
#[doc = "Field `IV31` writer - IV31"]
pub type IV31_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
#[doc = "Field `IV30` reader - IV30"]
pub type IV30_R = crate::BitReader<bool>;
#[doc = "Field `IV30` writer - IV30"]
pub type IV30_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
#[doc = "Field `IV29` reader - IV29"]
pub type IV29_R = crate::BitReader<bool>;
#[doc = "Field `IV29` writer - IV29"]
pub type IV29_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
#[doc = "Field `IV28` reader - IV28"]
pub type IV28_R = crate::BitReader<bool>;
#[doc = "Field `IV28` writer - IV28"]
pub type IV28_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
#[doc = "Field `IV27` reader - IV27"]
pub type IV27_R = crate::BitReader<bool>;
#[doc = "Field `IV27` writer - IV27"]
pub type IV27_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
#[doc = "Field `IV26` reader - IV26"]
pub type IV26_R = crate::BitReader<bool>;
#[doc = "Field `IV26` writer - IV26"]
pub type IV26_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
#[doc = "Field `IV25` reader - IV25"]
pub type IV25_R = crate::BitReader<bool>;
#[doc = "Field `IV25` writer - IV25"]
pub type IV25_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
#[doc = "Field `IV24` reader - IV24"]
pub type IV24_R = crate::BitReader<bool>;
#[doc = "Field `IV24` writer - IV24"]
pub type IV24_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
#[doc = "Field `IV23` reader - IV23"]
pub type IV23_R = crate::BitReader<bool>;
#[doc = "Field `IV23` writer - IV23"]
pub type IV23_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
#[doc = "Field `IV22` reader - IV22"]
pub type IV22_R = crate::BitReader<bool>;
#[doc = "Field `IV22` writer - IV22"]
pub type IV22_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
#[doc = "Field `IV21` reader - IV21"]
pub type IV21_R = crate::BitReader<bool>;
#[doc = "Field `IV21` writer - IV21"]
pub type IV21_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
#[doc = "Field `IV20` reader - IV20"]
pub type IV20_R = crate::BitReader<bool>;
#[doc = "Field `IV20` writer - IV20"]
pub type IV20_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
#[doc = "Field `IV19` reader - IV19"]
pub type IV19_R = crate::BitReader<bool>;
#[doc = "Field `IV19` writer - IV19"]
pub type IV19_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
#[doc = "Field `IV18` reader - IV18"]
pub type IV18_R = crate::BitReader<bool>;
#[doc = "Field `IV18` writer - IV18"]
pub type IV18_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
#[doc = "Field `IV17` reader - IV17"]
pub type IV17_R = crate::BitReader<bool>;
#[doc = "Field `IV17` writer - IV17"]
pub type IV17_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
#[doc = "Field `IV16` reader - IV16"]
pub type IV16_R = crate::BitReader<bool>;
#[doc = "Field `IV16` writer - IV16"]
pub type IV16_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
#[doc = "Field `IV15` reader - IV15"]
pub type IV15_R = crate::BitReader<bool>;
#[doc = "Field `IV15` writer - IV15"]
pub type IV15_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
#[doc = "Field `IV14` reader - IV14"]
pub type IV14_R = crate::BitReader<bool>;
#[doc = "Field `IV14` writer - IV14"]
pub type IV14_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
#[doc = "Field `IV13` reader - IV13"]
pub type IV13_R = crate::BitReader<bool>;
#[doc = "Field `IV13` writer - IV13"]
pub type IV13_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
#[doc = "Field `IV12` reader - IV12"]
pub type IV12_R = crate::BitReader<bool>;
#[doc = "Field `IV12` writer - IV12"]
pub type IV12_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
#[doc = "Field `IV11` reader - IV11"]
pub type IV11_R = crate::BitReader<bool>;
#[doc = "Field `IV11` writer - IV11"]
pub type IV11_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
#[doc = "Field `IV10` reader - IV10"]
pub type IV10_R = crate::BitReader<bool>;
#[doc = "Field `IV10` writer - IV10"]
pub type IV10_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
#[doc = "Field `IV9` reader - IV9"]
pub type IV9_R = crate::BitReader<bool>;
#[doc = "Field `IV9` writer - IV9"]
pub type IV9_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
#[doc = "Field `IV8` reader - IV8"]
pub type IV8_R = crate::BitReader<bool>;
#[doc = "Field `IV8` writer - IV8"]
pub type IV8_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
#[doc = "Field `IV7` reader - IV7"]
pub type IV7_R = crate::BitReader<bool>;
#[doc = "Field `IV7` writer - IV7"]
pub type IV7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
#[doc = "Field `IV6` reader - IV6"]
pub type IV6_R = crate::BitReader<bool>;
#[doc = "Field `IV6` writer - IV6"]
pub type IV6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
#[doc = "Field `IV5` reader - IV5"]
pub type IV5_R = crate::BitReader<bool>;
#[doc = "Field `IV5` writer - IV5"]
pub type IV5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
#[doc = "Field `IV4` reader - IV4"]
pub type IV4_R = crate::BitReader<bool>;
#[doc = "Field `IV4` writer - IV4"]
pub type IV4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
#[doc = "Field `IV3` reader - IV3"]
pub type IV3_R = crate::BitReader<bool>;
#[doc = "Field `IV3` writer - IV3"]
pub type IV3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
#[doc = "Field `IV2` reader - IV2"]
pub type IV2_R = crate::BitReader<bool>;
#[doc = "Field `IV2` writer - IV2"]
pub type IV2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
#[doc = "Field `IV1` reader - IV1"]
pub type IV1_R = crate::BitReader<bool>;
#[doc = "Field `IV1` writer - IV1"]
pub type IV1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
#[doc = "Field `IV0` reader - IV0"]
pub type IV0_R = crate::BitReader<bool>;
#[doc = "Field `IV0` writer - IV0"]
pub type IV0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_IV0LR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - IV31"]
    #[inline(always)]
    pub fn iv31(&self) -> IV31_R {
        IV31_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IV30"]
    #[inline(always)]
    pub fn iv30(&self) -> IV30_R {
        IV30_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IV29"]
    #[inline(always)]
    pub fn iv29(&self) -> IV29_R {
        IV29_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IV28"]
    #[inline(always)]
    pub fn iv28(&self) -> IV28_R {
        IV28_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IV27"]
    #[inline(always)]
    pub fn iv27(&self) -> IV27_R {
        IV27_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IV26"]
    #[inline(always)]
    pub fn iv26(&self) -> IV26_R {
        IV26_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IV25"]
    #[inline(always)]
    pub fn iv25(&self) -> IV25_R {
        IV25_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IV24"]
    #[inline(always)]
    pub fn iv24(&self) -> IV24_R {
        IV24_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IV23"]
    #[inline(always)]
    pub fn iv23(&self) -> IV23_R {
        IV23_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IV22"]
    #[inline(always)]
    pub fn iv22(&self) -> IV22_R {
        IV22_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IV21"]
    #[inline(always)]
    pub fn iv21(&self) -> IV21_R {
        IV21_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - IV20"]
    #[inline(always)]
    pub fn iv20(&self) -> IV20_R {
        IV20_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - IV19"]
    #[inline(always)]
    pub fn iv19(&self) -> IV19_R {
        IV19_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - IV18"]
    #[inline(always)]
    pub fn iv18(&self) -> IV18_R {
        IV18_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - IV17"]
    #[inline(always)]
    pub fn iv17(&self) -> IV17_R {
        IV17_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - IV16"]
    #[inline(always)]
    pub fn iv16(&self) -> IV16_R {
        IV16_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - IV15"]
    #[inline(always)]
    pub fn iv15(&self) -> IV15_R {
        IV15_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - IV14"]
    #[inline(always)]
    pub fn iv14(&self) -> IV14_R {
        IV14_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - IV13"]
    #[inline(always)]
    pub fn iv13(&self) -> IV13_R {
        IV13_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - IV12"]
    #[inline(always)]
    pub fn iv12(&self) -> IV12_R {
        IV12_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - IV11"]
    #[inline(always)]
    pub fn iv11(&self) -> IV11_R {
        IV11_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - IV10"]
    #[inline(always)]
    pub fn iv10(&self) -> IV10_R {
        IV10_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - IV9"]
    #[inline(always)]
    pub fn iv9(&self) -> IV9_R {
        IV9_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - IV8"]
    #[inline(always)]
    pub fn iv8(&self) -> IV8_R {
        IV8_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - IV7"]
    #[inline(always)]
    pub fn iv7(&self) -> IV7_R {
        IV7_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - IV6"]
    #[inline(always)]
    pub fn iv6(&self) -> IV6_R {
        IV6_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - IV5"]
    #[inline(always)]
    pub fn iv5(&self) -> IV5_R {
        IV5_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - IV4"]
    #[inline(always)]
    pub fn iv4(&self) -> IV4_R {
        IV4_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - IV3"]
    #[inline(always)]
    pub fn iv3(&self) -> IV3_R {
        IV3_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - IV2"]
    #[inline(always)]
    pub fn iv2(&self) -> IV2_R {
        IV2_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - IV1"]
    #[inline(always)]
    pub fn iv1(&self) -> IV1_R {
        IV1_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - IV0"]
    #[inline(always)]
    pub fn iv0(&self) -> IV0_R {
        IV0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IV31"]
    #[inline(always)]
    #[must_use]
    pub fn iv31(&mut self) -> IV31_W<0> {
        IV31_W::new(self)
    }
    #[doc = "Bit 1 - IV30"]
    #[inline(always)]
    #[must_use]
    pub fn iv30(&mut self) -> IV30_W<1> {
        IV30_W::new(self)
    }
    #[doc = "Bit 2 - IV29"]
    #[inline(always)]
    #[must_use]
    pub fn iv29(&mut self) -> IV29_W<2> {
        IV29_W::new(self)
    }
    #[doc = "Bit 3 - IV28"]
    #[inline(always)]
    #[must_use]
    pub fn iv28(&mut self) -> IV28_W<3> {
        IV28_W::new(self)
    }
    #[doc = "Bit 4 - IV27"]
    #[inline(always)]
    #[must_use]
    pub fn iv27(&mut self) -> IV27_W<4> {
        IV27_W::new(self)
    }
    #[doc = "Bit 5 - IV26"]
    #[inline(always)]
    #[must_use]
    pub fn iv26(&mut self) -> IV26_W<5> {
        IV26_W::new(self)
    }
    #[doc = "Bit 6 - IV25"]
    #[inline(always)]
    #[must_use]
    pub fn iv25(&mut self) -> IV25_W<6> {
        IV25_W::new(self)
    }
    #[doc = "Bit 7 - IV24"]
    #[inline(always)]
    #[must_use]
    pub fn iv24(&mut self) -> IV24_W<7> {
        IV24_W::new(self)
    }
    #[doc = "Bit 8 - IV23"]
    #[inline(always)]
    #[must_use]
    pub fn iv23(&mut self) -> IV23_W<8> {
        IV23_W::new(self)
    }
    #[doc = "Bit 9 - IV22"]
    #[inline(always)]
    #[must_use]
    pub fn iv22(&mut self) -> IV22_W<9> {
        IV22_W::new(self)
    }
    #[doc = "Bit 10 - IV21"]
    #[inline(always)]
    #[must_use]
    pub fn iv21(&mut self) -> IV21_W<10> {
        IV21_W::new(self)
    }
    #[doc = "Bit 11 - IV20"]
    #[inline(always)]
    #[must_use]
    pub fn iv20(&mut self) -> IV20_W<11> {
        IV20_W::new(self)
    }
    #[doc = "Bit 12 - IV19"]
    #[inline(always)]
    #[must_use]
    pub fn iv19(&mut self) -> IV19_W<12> {
        IV19_W::new(self)
    }
    #[doc = "Bit 13 - IV18"]
    #[inline(always)]
    #[must_use]
    pub fn iv18(&mut self) -> IV18_W<13> {
        IV18_W::new(self)
    }
    #[doc = "Bit 14 - IV17"]
    #[inline(always)]
    #[must_use]
    pub fn iv17(&mut self) -> IV17_W<14> {
        IV17_W::new(self)
    }
    #[doc = "Bit 15 - IV16"]
    #[inline(always)]
    #[must_use]
    pub fn iv16(&mut self) -> IV16_W<15> {
        IV16_W::new(self)
    }
    #[doc = "Bit 16 - IV15"]
    #[inline(always)]
    #[must_use]
    pub fn iv15(&mut self) -> IV15_W<16> {
        IV15_W::new(self)
    }
    #[doc = "Bit 17 - IV14"]
    #[inline(always)]
    #[must_use]
    pub fn iv14(&mut self) -> IV14_W<17> {
        IV14_W::new(self)
    }
    #[doc = "Bit 18 - IV13"]
    #[inline(always)]
    #[must_use]
    pub fn iv13(&mut self) -> IV13_W<18> {
        IV13_W::new(self)
    }
    #[doc = "Bit 19 - IV12"]
    #[inline(always)]
    #[must_use]
    pub fn iv12(&mut self) -> IV12_W<19> {
        IV12_W::new(self)
    }
    #[doc = "Bit 20 - IV11"]
    #[inline(always)]
    #[must_use]
    pub fn iv11(&mut self) -> IV11_W<20> {
        IV11_W::new(self)
    }
    #[doc = "Bit 21 - IV10"]
    #[inline(always)]
    #[must_use]
    pub fn iv10(&mut self) -> IV10_W<21> {
        IV10_W::new(self)
    }
    #[doc = "Bit 22 - IV9"]
    #[inline(always)]
    #[must_use]
    pub fn iv9(&mut self) -> IV9_W<22> {
        IV9_W::new(self)
    }
    #[doc = "Bit 23 - IV8"]
    #[inline(always)]
    #[must_use]
    pub fn iv8(&mut self) -> IV8_W<23> {
        IV8_W::new(self)
    }
    #[doc = "Bit 24 - IV7"]
    #[inline(always)]
    #[must_use]
    pub fn iv7(&mut self) -> IV7_W<24> {
        IV7_W::new(self)
    }
    #[doc = "Bit 25 - IV6"]
    #[inline(always)]
    #[must_use]
    pub fn iv6(&mut self) -> IV6_W<25> {
        IV6_W::new(self)
    }
    #[doc = "Bit 26 - IV5"]
    #[inline(always)]
    #[must_use]
    pub fn iv5(&mut self) -> IV5_W<26> {
        IV5_W::new(self)
    }
    #[doc = "Bit 27 - IV4"]
    #[inline(always)]
    #[must_use]
    pub fn iv4(&mut self) -> IV4_W<27> {
        IV4_W::new(self)
    }
    #[doc = "Bit 28 - IV3"]
    #[inline(always)]
    #[must_use]
    pub fn iv3(&mut self) -> IV3_W<28> {
        IV3_W::new(self)
    }
    #[doc = "Bit 29 - IV2"]
    #[inline(always)]
    #[must_use]
    pub fn iv2(&mut self) -> IV2_W<29> {
        IV2_W::new(self)
    }
    #[doc = "Bit 30 - IV1"]
    #[inline(always)]
    #[must_use]
    pub fn iv1(&mut self) -> IV1_W<30> {
        IV1_W::new(self)
    }
    #[doc = "Bit 31 - IV0"]
    #[inline(always)]
    #[must_use]
    pub fn iv0(&mut self) -> IV0_W<31> {
        IV0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The CRYP_IV0...1(L/R)R are the left-word and right-word registers for the initialization vector (64 bits for DES/TDES and 128 bits for AES). For more information refer to Section39.3.18: CRYP initialization vector registers. IV0 is the leftmost bit whereas IV63 (DES, TDES) or IV127 (AES) are the rightmost bits of the initialization vector. IV1(L/R)R is used only in the AES. Only CRYP_IV0(L/R) is used in DES/TDES. Write access to these registers are disregarded when the cryptographic processor is busy (bit BUSY = 1 in the CRYP_SR register).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_iv0lr](index.html) module"]
pub struct CRYP_IV0LR_SPEC;
impl crate::RegisterSpec for CRYP_IV0LR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cryp_iv0lr::R](R) reader structure"]
impl crate::Readable for CRYP_IV0LR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cryp_iv0lr::W](W) writer structure"]
impl crate::Writable for CRYP_IV0LR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRYP_IV0LR to value 0"]
impl crate::Resettable for CRYP_IV0LR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
