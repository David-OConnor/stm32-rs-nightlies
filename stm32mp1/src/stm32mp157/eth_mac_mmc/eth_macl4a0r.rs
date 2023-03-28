#[doc = "Register `ETH_MACL4A0R` reader"]
pub struct R(crate::R<ETH_MACL4A0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACL4A0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACL4A0R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACL4A0R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACL4A0R` writer"]
pub struct W(crate::W<ETH_MACL4A0R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACL4A0R_SPEC>;
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
impl From<crate::W<ETH_MACL4A0R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACL4A0R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L4SP0` reader - L4SP0"]
pub type L4SP0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `L4SP0` writer - L4SP0"]
pub type L4SP0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACL4A0R_SPEC, u16, u16, 16, O>;
#[doc = "Field `L4DP0` reader - L4DP0"]
pub type L4DP0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `L4DP0` writer - L4DP0"]
pub type L4DP0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACL4A0R_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - L4SP0"]
    #[inline(always)]
    pub fn l4sp0(&self) -> L4SP0_R {
        L4SP0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - L4DP0"]
    #[inline(always)]
    pub fn l4dp0(&self) -> L4DP0_R {
        L4DP0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - L4SP0"]
    #[inline(always)]
    #[must_use]
    pub fn l4sp0(&mut self) -> L4SP0_W<0> {
        L4SP0_W::new(self)
    }
    #[doc = "Bits 16:31 - L4DP0"]
    #[inline(always)]
    #[must_use]
    pub fn l4dp0(&mut self) -> L4DP0_W<16> {
        L4DP0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer4 address filter 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macl4a0r](index.html) module"]
pub struct ETH_MACL4A0R_SPEC;
impl crate::RegisterSpec for ETH_MACL4A0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macl4a0r::R](R) reader structure"]
impl crate::Readable for ETH_MACL4A0R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macl4a0r::W](W) writer structure"]
impl crate::Writable for ETH_MACL4A0R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACL4A0R to value 0"]
impl crate::Resettable for ETH_MACL4A0R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
