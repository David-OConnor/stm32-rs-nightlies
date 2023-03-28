#[doc = "Register `TIR` reader"]
pub struct R(crate::R<TIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIR` writer"]
pub struct W(crate::W<TIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIR_SPEC>;
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
impl From<crate::W<TIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXRQ` reader - TXRQ"]
pub type TXRQ_R = crate::BitReader<bool>;
#[doc = "Field `TXRQ` writer - TXRQ"]
pub type TXRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIR_SPEC, bool, O>;
#[doc = "Field `RTR` reader - RTR"]
pub type RTR_R = crate::BitReader<bool>;
#[doc = "Field `RTR` writer - RTR"]
pub type RTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIR_SPEC, bool, O>;
#[doc = "Field `IDE` reader - IDE"]
pub type IDE_R = crate::BitReader<bool>;
#[doc = "Field `IDE` writer - IDE"]
pub type IDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIR_SPEC, bool, O>;
#[doc = "Field `EXID` reader - EXID"]
pub type EXID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EXID` writer - EXID"]
pub type EXID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIR_SPEC, u32, u32, 18, O>;
#[doc = "Field `STID` reader - STID"]
pub type STID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STID` writer - STID"]
pub type STID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIR_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bit 0 - TXRQ"]
    #[inline(always)]
    pub fn txrq(&self) -> TXRQ_R {
        TXRQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTR"]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IDE"]
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:20 - EXID"]
    #[inline(always)]
    pub fn exid(&self) -> EXID_R {
        EXID_R::new((self.bits >> 3) & 0x0003_ffff)
    }
    #[doc = "Bits 21:31 - STID"]
    #[inline(always)]
    pub fn stid(&self) -> STID_R {
        STID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - TXRQ"]
    #[inline(always)]
    #[must_use]
    pub fn txrq(&mut self) -> TXRQ_W<0> {
        TXRQ_W::new(self)
    }
    #[doc = "Bit 1 - RTR"]
    #[inline(always)]
    #[must_use]
    pub fn rtr(&mut self) -> RTR_W<1> {
        RTR_W::new(self)
    }
    #[doc = "Bit 2 - IDE"]
    #[inline(always)]
    #[must_use]
    pub fn ide(&mut self) -> IDE_W<2> {
        IDE_W::new(self)
    }
    #[doc = "Bits 3:20 - EXID"]
    #[inline(always)]
    #[must_use]
    pub fn exid(&mut self) -> EXID_W<3> {
        EXID_W::new(self)
    }
    #[doc = "Bits 21:31 - STID"]
    #[inline(always)]
    #[must_use]
    pub fn stid(&mut self) -> STID_W<21> {
        STID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN_TI0R\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tir](index.html) module"]
pub struct TIR_SPEC;
impl crate::RegisterSpec for TIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tir::R](R) reader structure"]
impl crate::Readable for TIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tir::W](W) writer structure"]
impl crate::Writable for TIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIR to value 0"]
impl crate::Resettable for TIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
