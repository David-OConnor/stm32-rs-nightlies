#[doc = "Register `OPAMP1_HSOTR` reader"]
pub struct R(crate::R<OPAMP1_HSOTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPAMP1_HSOTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPAMP1_HSOTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPAMP1_HSOTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPAMP1_HSOTR` writer"]
pub struct W(crate::W<OPAMP1_HSOTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPAMP1_HSOTR_SPEC>;
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
impl From<crate::W<OPAMP1_HSOTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPAMP1_HSOTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIMHSOFFSETN` reader - High-speed mode trim for NMOS differential pairs"]
pub type TRIMHSOFFSETN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIMHSOFFSETN` writer - High-speed mode trim for NMOS differential pairs"]
pub type TRIMHSOFFSETN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OPAMP1_HSOTR_SPEC, u8, u8, 5, O>;
#[doc = "Field `TRIMHSOFFSETP` reader - High-speed mode trim for PMOS differential pairs"]
pub type TRIMHSOFFSETP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIMHSOFFSETP` writer - High-speed mode trim for PMOS differential pairs"]
pub type TRIMHSOFFSETP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OPAMP1_HSOTR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - High-speed mode trim for NMOS differential pairs"]
    #[inline(always)]
    pub fn trimhsoffsetn(&self) -> TRIMHSOFFSETN_R {
        TRIMHSOFFSETN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - High-speed mode trim for PMOS differential pairs"]
    #[inline(always)]
    pub fn trimhsoffsetp(&self) -> TRIMHSOFFSETP_R {
        TRIMHSOFFSETP_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - High-speed mode trim for NMOS differential pairs"]
    #[inline(always)]
    #[must_use]
    pub fn trimhsoffsetn(&mut self) -> TRIMHSOFFSETN_W<0> {
        TRIMHSOFFSETN_W::new(self)
    }
    #[doc = "Bits 8:12 - High-speed mode trim for PMOS differential pairs"]
    #[inline(always)]
    #[must_use]
    pub fn trimhsoffsetp(&mut self) -> TRIMHSOFFSETP_W<8> {
        TRIMHSOFFSETP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OPAMP1 trimming register in high-speed mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opamp1_hsotr](index.html) module"]
pub struct OPAMP1_HSOTR_SPEC;
impl crate::RegisterSpec for OPAMP1_HSOTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opamp1_hsotr::R](R) reader structure"]
impl crate::Readable for OPAMP1_HSOTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opamp1_hsotr::W](W) writer structure"]
impl crate::Writable for OPAMP1_HSOTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPAMP1_HSOTR to value 0"]
impl crate::Resettable for OPAMP1_HSOTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
