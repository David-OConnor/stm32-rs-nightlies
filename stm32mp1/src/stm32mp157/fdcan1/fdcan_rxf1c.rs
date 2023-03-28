#[doc = "Register `FDCAN_RXF1C` reader"]
pub struct R(crate::R<FDCAN_RXF1C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_RXF1C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_RXF1C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_RXF1C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_RXF1C` writer"]
pub struct W(crate::W<FDCAN_RXF1C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_RXF1C_SPEC>;
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
impl From<crate::W<FDCAN_RXF1C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_RXF1C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `F1SA` reader - F1SA"]
pub type F1SA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `F1SA` writer - F1SA"]
pub type F1SA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_RXF1C_SPEC, u16, u16, 14, O>;
#[doc = "Field `F1S` reader - F1S"]
pub type F1S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `F1S` writer - F1S"]
pub type F1S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_RXF1C_SPEC, u8, u8, 7, O>;
#[doc = "Field `F1WM` reader - F1WM"]
pub type F1WM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `F1WM` writer - F1WM"]
pub type F1WM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_RXF1C_SPEC, u8, u8, 7, O>;
#[doc = "Field `F1OM` reader - F1OM"]
pub type F1OM_R = crate::BitReader<bool>;
#[doc = "Field `F1OM` writer - F1OM"]
pub type F1OM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_RXF1C_SPEC, bool, O>;
impl R {
    #[doc = "Bits 2:15 - F1SA"]
    #[inline(always)]
    pub fn f1sa(&self) -> F1SA_R {
        F1SA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:22 - F1S"]
    #[inline(always)]
    pub fn f1s(&self) -> F1S_R {
        F1S_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - F1WM"]
    #[inline(always)]
    pub fn f1wm(&self) -> F1WM_R {
        F1WM_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - F1OM"]
    #[inline(always)]
    pub fn f1om(&self) -> F1OM_R {
        F1OM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - F1SA"]
    #[inline(always)]
    #[must_use]
    pub fn f1sa(&mut self) -> F1SA_W<2> {
        F1SA_W::new(self)
    }
    #[doc = "Bits 16:22 - F1S"]
    #[inline(always)]
    #[must_use]
    pub fn f1s(&mut self) -> F1S_W<16> {
        F1S_W::new(self)
    }
    #[doc = "Bits 24:30 - F1WM"]
    #[inline(always)]
    #[must_use]
    pub fn f1wm(&mut self) -> F1WM_W<24> {
        F1WM_W::new(self)
    }
    #[doc = "Bit 31 - F1OM"]
    #[inline(always)]
    #[must_use]
    pub fn f1om(&mut self) -> F1OM_W<31> {
        F1OM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Rx FIFO 1 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_rxf1c](index.html) module"]
pub struct FDCAN_RXF1C_SPEC;
impl crate::RegisterSpec for FDCAN_RXF1C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_rxf1c::R](R) reader structure"]
impl crate::Readable for FDCAN_RXF1C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_rxf1c::W](W) writer structure"]
impl crate::Writable for FDCAN_RXF1C_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_RXF1C to value 0"]
impl crate::Resettable for FDCAN_RXF1C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
