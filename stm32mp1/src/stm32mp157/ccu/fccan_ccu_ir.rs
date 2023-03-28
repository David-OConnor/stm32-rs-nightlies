#[doc = "Register `FCCAN_CCU_IR` reader"]
pub struct R(crate::R<FCCAN_CCU_IR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCCAN_CCU_IR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCCAN_CCU_IR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCCAN_CCU_IR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCCAN_CCU_IR` writer"]
pub struct W(crate::W<FCCAN_CCU_IR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCCAN_CCU_IR_SPEC>;
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
impl From<crate::W<FCCAN_CCU_IR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCCAN_CCU_IR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CWE` reader - CWE"]
pub type CWE_R = crate::BitReader<bool>;
#[doc = "Field `CWE` writer - CWE"]
pub type CWE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCCAN_CCU_IR_SPEC, bool, O>;
#[doc = "Field `CSC` reader - CSC"]
pub type CSC_R = crate::BitReader<bool>;
#[doc = "Field `CSC` writer - CSC"]
pub type CSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCCAN_CCU_IR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CWE"]
    #[inline(always)]
    pub fn cwe(&self) -> CWE_R {
        CWE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CSC"]
    #[inline(always)]
    pub fn csc(&self) -> CSC_R {
        CSC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CWE"]
    #[inline(always)]
    #[must_use]
    pub fn cwe(&mut self) -> CWE_W<0> {
        CWE_W::new(self)
    }
    #[doc = "Bit 1 - CSC"]
    #[inline(always)]
    #[must_use]
    pub fn csc(&mut self) -> CSC_W<1> {
        CSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The flags are set when one of the listed conditions is detected (edge-sensitive). The flags remain set until the Host clears them. A flag is cleared by writing a 1 to the corresponding bit position. Writing a 0 has no effect. A hard reset will clear the register. The configuration of CCU_IE controls whether an interrupt is generated or not.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccan_ccu_ir](index.html) module"]
pub struct FCCAN_CCU_IR_SPEC;
impl crate::RegisterSpec for FCCAN_CCU_IR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fccan_ccu_ir::R](R) reader structure"]
impl crate::Readable for FCCAN_CCU_IR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fccan_ccu_ir::W](W) writer structure"]
impl crate::Writable for FCCAN_CCU_IR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCCAN_CCU_IR to value 0"]
impl crate::Resettable for FCCAN_CCU_IR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
