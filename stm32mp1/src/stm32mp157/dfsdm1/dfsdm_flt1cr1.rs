#[doc = "Register `DFSDM_FLT1CR1` reader"]
pub struct R(crate::R<DFSDM_FLT1CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_FLT1CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_FLT1CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_FLT1CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFSDM_FLT1CR1` writer"]
pub struct W(crate::W<DFSDM_FLT1CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFSDM_FLT1CR1_SPEC>;
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
impl From<crate::W<DFSDM_FLT1CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFSDM_FLT1CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DFEN` reader - DFEN"]
pub type DFEN_R = crate::BitReader<bool>;
#[doc = "Field `DFEN` writer - DFEN"]
pub type DFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSDM_FLT1CR1_SPEC, bool, O>;
#[doc = "Field `JSWSTART` reader - JSWSTART"]
pub type JSWSTART_R = crate::BitReader<bool>;
#[doc = "Field `JSWSTART` writer - JSWSTART"]
pub type JSWSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSDM_FLT1CR1_SPEC, bool, O>;
#[doc = "Field `JSYNC` reader - JSYNC"]
pub type JSYNC_R = crate::BitReader<bool>;
#[doc = "Field `JSYNC` writer - JSYNC"]
pub type JSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSDM_FLT1CR1_SPEC, bool, O>;
#[doc = "Field `JSCAN` reader - JSCAN"]
pub type JSCAN_R = crate::BitReader<bool>;
#[doc = "Field `JSCAN` writer - JSCAN"]
pub type JSCAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSDM_FLT1CR1_SPEC, bool, O>;
#[doc = "Field `JDMAEN` reader - JDMAEN"]
pub type JDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `JDMAEN` writer - JDMAEN"]
pub type JDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSDM_FLT1CR1_SPEC, bool, O>;
#[doc = "Field `JEXTSEL` reader - JEXTSEL"]
pub type JEXTSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JEXTSEL` writer - JEXTSEL"]
pub type JEXTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFSDM_FLT1CR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `JEXTEN` reader - JEXTEN"]
pub type JEXTEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JEXTEN` writer - JEXTEN"]
pub type JEXTEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFSDM_FLT1CR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `RSWSTART` reader - RSWSTART"]
pub type RSWSTART_R = crate::BitReader<bool>;
#[doc = "Field `RSWSTART` writer - RSWSTART"]
pub type RSWSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSDM_FLT1CR1_SPEC, bool, O>;
#[doc = "Field `RCONT` reader - RCONT"]
pub type RCONT_R = crate::BitReader<bool>;
#[doc = "Field `RCONT` writer - RCONT"]
pub type RCONT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSDM_FLT1CR1_SPEC, bool, O>;
#[doc = "Field `RSYNC` reader - RSYNC"]
pub type RSYNC_R = crate::BitReader<bool>;
#[doc = "Field `RSYNC` writer - RSYNC"]
pub type RSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSDM_FLT1CR1_SPEC, bool, O>;
#[doc = "Field `RDMAEN` reader - RDMAEN"]
pub type RDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `RDMAEN` writer - RDMAEN"]
pub type RDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSDM_FLT1CR1_SPEC, bool, O>;
#[doc = "Field `RCH` reader - RCH"]
pub type RCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCH` writer - RCH"]
pub type RCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFSDM_FLT1CR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `FAST` reader - FAST"]
pub type FAST_R = crate::BitReader<bool>;
#[doc = "Field `FAST` writer - FAST"]
pub type FAST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSDM_FLT1CR1_SPEC, bool, O>;
#[doc = "Field `AWFSEL` reader - AWFSEL"]
pub type AWFSEL_R = crate::BitReader<bool>;
#[doc = "Field `AWFSEL` writer - AWFSEL"]
pub type AWFSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSDM_FLT1CR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DFEN"]
    #[inline(always)]
    pub fn dfen(&self) -> DFEN_R {
        DFEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - JSWSTART"]
    #[inline(always)]
    pub fn jswstart(&self) -> JSWSTART_R {
        JSWSTART_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - JSYNC"]
    #[inline(always)]
    pub fn jsync(&self) -> JSYNC_R {
        JSYNC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - JSCAN"]
    #[inline(always)]
    pub fn jscan(&self) -> JSCAN_R {
        JSCAN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - JDMAEN"]
    #[inline(always)]
    pub fn jdmaen(&self) -> JDMAEN_R {
        JDMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:12 - JEXTSEL"]
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - JEXTEN"]
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 17 - RSWSTART"]
    #[inline(always)]
    pub fn rswstart(&self) -> RSWSTART_R {
        RSWSTART_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RCONT"]
    #[inline(always)]
    pub fn rcont(&self) -> RCONT_R {
        RCONT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RSYNC"]
    #[inline(always)]
    pub fn rsync(&self) -> RSYNC_R {
        RSYNC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - RDMAEN"]
    #[inline(always)]
    pub fn rdmaen(&self) -> RDMAEN_R {
        RDMAEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:26 - RCH"]
    #[inline(always)]
    pub fn rch(&self) -> RCH_R {
        RCH_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 29 - FAST"]
    #[inline(always)]
    pub fn fast(&self) -> FAST_R {
        FAST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - AWFSEL"]
    #[inline(always)]
    pub fn awfsel(&self) -> AWFSEL_R {
        AWFSEL_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DFEN"]
    #[inline(always)]
    #[must_use]
    pub fn dfen(&mut self) -> DFEN_W<0> {
        DFEN_W::new(self)
    }
    #[doc = "Bit 1 - JSWSTART"]
    #[inline(always)]
    #[must_use]
    pub fn jswstart(&mut self) -> JSWSTART_W<1> {
        JSWSTART_W::new(self)
    }
    #[doc = "Bit 3 - JSYNC"]
    #[inline(always)]
    #[must_use]
    pub fn jsync(&mut self) -> JSYNC_W<3> {
        JSYNC_W::new(self)
    }
    #[doc = "Bit 4 - JSCAN"]
    #[inline(always)]
    #[must_use]
    pub fn jscan(&mut self) -> JSCAN_W<4> {
        JSCAN_W::new(self)
    }
    #[doc = "Bit 5 - JDMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn jdmaen(&mut self) -> JDMAEN_W<5> {
        JDMAEN_W::new(self)
    }
    #[doc = "Bits 8:12 - JEXTSEL"]
    #[inline(always)]
    #[must_use]
    pub fn jextsel(&mut self) -> JEXTSEL_W<8> {
        JEXTSEL_W::new(self)
    }
    #[doc = "Bits 13:14 - JEXTEN"]
    #[inline(always)]
    #[must_use]
    pub fn jexten(&mut self) -> JEXTEN_W<13> {
        JEXTEN_W::new(self)
    }
    #[doc = "Bit 17 - RSWSTART"]
    #[inline(always)]
    #[must_use]
    pub fn rswstart(&mut self) -> RSWSTART_W<17> {
        RSWSTART_W::new(self)
    }
    #[doc = "Bit 18 - RCONT"]
    #[inline(always)]
    #[must_use]
    pub fn rcont(&mut self) -> RCONT_W<18> {
        RCONT_W::new(self)
    }
    #[doc = "Bit 19 - RSYNC"]
    #[inline(always)]
    #[must_use]
    pub fn rsync(&mut self) -> RSYNC_W<19> {
        RSYNC_W::new(self)
    }
    #[doc = "Bit 21 - RDMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn rdmaen(&mut self) -> RDMAEN_W<21> {
        RDMAEN_W::new(self)
    }
    #[doc = "Bits 24:26 - RCH"]
    #[inline(always)]
    #[must_use]
    pub fn rch(&mut self) -> RCH_W<24> {
        RCH_W::new(self)
    }
    #[doc = "Bit 29 - FAST"]
    #[inline(always)]
    #[must_use]
    pub fn fast(&mut self) -> FAST_W<29> {
        FAST_W::new(self)
    }
    #[doc = "Bit 30 - AWFSEL"]
    #[inline(always)]
    #[must_use]
    pub fn awfsel(&mut self) -> AWFSEL_W<30> {
        AWFSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFSDM filter 1 control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt1cr1](index.html) module"]
pub struct DFSDM_FLT1CR1_SPEC;
impl crate::RegisterSpec for DFSDM_FLT1CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm_flt1cr1::R](R) reader structure"]
impl crate::Readable for DFSDM_FLT1CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt1cr1::W](W) writer structure"]
impl crate::Writable for DFSDM_FLT1CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFSDM_FLT1CR1 to value 0"]
impl crate::Resettable for DFSDM_FLT1CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
