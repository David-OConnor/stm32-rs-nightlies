#[doc = "Register `ETH_MACSTSUR` reader"]
pub struct R(crate::R<ETH_MACSTSUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACSTSUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACSTSUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACSTSUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACSTSUR` writer"]
pub struct W(crate::W<ETH_MACSTSUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACSTSUR_SPEC>;
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
impl From<crate::W<ETH_MACSTSUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACSTSUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSS` reader - TSS"]
pub type TSS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TSS` writer - TSS"]
pub type TSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACSTSUR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - TSS"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TSS"]
    #[inline(always)]
    #[must_use]
    pub fn tss(&mut self) -> TSS_W<0> {
        TSS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The System Time Seconds Update register, along with the System Time Nanoseconds Update register, initializes or updates the system time maintained by the MAC. You must write both registers before setting the TSINIT or TSUPDT bits in ETH_MACTSCR register. This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macstsur](index.html) module"]
pub struct ETH_MACSTSUR_SPEC;
impl crate::RegisterSpec for ETH_MACSTSUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macstsur::R](R) reader structure"]
impl crate::Readable for ETH_MACSTSUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macstsur::W](W) writer structure"]
impl crate::Writable for ETH_MACSTSUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACSTSUR to value 0"]
impl crate::Resettable for ETH_MACSTSUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
