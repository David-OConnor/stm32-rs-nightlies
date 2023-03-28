#[doc = "Register `FDCAN_RXF0S` reader"]
pub struct R(crate::R<FDCAN_RXF0S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_RXF0S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_RXF0S_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_RXF0S_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_RXF0S` writer"]
pub struct W(crate::W<FDCAN_RXF0S_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_RXF0S_SPEC>;
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
impl From<crate::W<FDCAN_RXF0S_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_RXF0S_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `F0FL` reader - Rx FIFO 0 Fill Level"]
pub type F0FL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `F0FL` writer - Rx FIFO 0 Fill Level"]
pub type F0FL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_RXF0S_SPEC, u8, u8, 4, O>;
#[doc = "Field `F0GI` reader - Rx FIFO 0 Get Index"]
pub type F0GI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `F0GI` writer - Rx FIFO 0 Get Index"]
pub type F0GI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_RXF0S_SPEC, u8, u8, 2, O>;
#[doc = "Field `F0PI` reader - Rx FIFO 0 Put Index"]
pub type F0PI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `F0PI` writer - Rx FIFO 0 Put Index"]
pub type F0PI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_RXF0S_SPEC, u8, u8, 2, O>;
#[doc = "Field `F0F` reader - Rx FIFO 0 Full"]
pub type F0F_R = crate::BitReader<bool>;
#[doc = "Field `F0F` writer - Rx FIFO 0 Full"]
pub type F0F_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_RXF0S_SPEC, bool, O>;
#[doc = "Field `RF0L` reader - Rx FIFO 0 Message Lost"]
pub type RF0L_R = crate::BitReader<bool>;
#[doc = "Field `RF0L` writer - Rx FIFO 0 Message Lost"]
pub type RF0L_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_RXF0S_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Rx FIFO 0 Fill Level"]
    #[inline(always)]
    pub fn f0fl(&self) -> F0FL_R {
        F0FL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Rx FIFO 0 Get Index"]
    #[inline(always)]
    pub fn f0gi(&self) -> F0GI_R {
        F0GI_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Rx FIFO 0 Put Index"]
    #[inline(always)]
    pub fn f0pi(&self) -> F0PI_R {
        F0PI_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 0 Full"]
    #[inline(always)]
    pub fn f0f(&self) -> F0F_R {
        F0F_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 0 Message Lost"]
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Rx FIFO 0 Fill Level"]
    #[inline(always)]
    #[must_use]
    pub fn f0fl(&mut self) -> F0FL_W<0> {
        F0FL_W::new(self)
    }
    #[doc = "Bits 8:9 - Rx FIFO 0 Get Index"]
    #[inline(always)]
    #[must_use]
    pub fn f0gi(&mut self) -> F0GI_W<8> {
        F0GI_W::new(self)
    }
    #[doc = "Bits 16:17 - Rx FIFO 0 Put Index"]
    #[inline(always)]
    #[must_use]
    pub fn f0pi(&mut self) -> F0PI_W<16> {
        F0PI_W::new(self)
    }
    #[doc = "Bit 24 - Rx FIFO 0 Full"]
    #[inline(always)]
    #[must_use]
    pub fn f0f(&mut self) -> F0F_W<24> {
        F0F_W::new(self)
    }
    #[doc = "Bit 25 - Rx FIFO 0 Message Lost"]
    #[inline(always)]
    #[must_use]
    pub fn rf0l(&mut self) -> RF0L_W<25> {
        RF0L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Rx FIFO 0 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_rxf0s](index.html) module"]
pub struct FDCAN_RXF0S_SPEC;
impl crate::RegisterSpec for FDCAN_RXF0S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_rxf0s::R](R) reader structure"]
impl crate::Readable for FDCAN_RXF0S_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_rxf0s::W](W) writer structure"]
impl crate::Writable for FDCAN_RXF0S_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_RXF0S to value 0"]
impl crate::Resettable for FDCAN_RXF0S_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
