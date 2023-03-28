#[doc = "Register `ETH_MACMDIODR` reader"]
pub struct R(crate::R<ETH_MACMDIODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACMDIODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACMDIODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACMDIODR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACMDIODR` writer"]
pub struct W(crate::W<ETH_MACMDIODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACMDIODR_SPEC>;
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
impl From<crate::W<ETH_MACMDIODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACMDIODR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MD` reader - MII Data This field contains the 16-bit data value read from the PHY after a Management Read operation or the 16-bit data value to be written to the PHY before a Management Write operation."]
pub type MD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MD` writer - MII Data This field contains the 16-bit data value read from the PHY after a Management Read operation or the 16-bit data value to be written to the PHY before a Management Write operation."]
pub type MD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACMDIODR_SPEC, u16, u16, 16, O>;
#[doc = "Field `RA` reader - Register Address This field is valid only when C45E is set. It contains the Register Address in the PHY to which the MDIO frame is intended for."]
pub type RA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RA` writer - Register Address This field is valid only when C45E is set. It contains the Register Address in the PHY to which the MDIO frame is intended for."]
pub type RA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACMDIODR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - MII Data This field contains the 16-bit data value read from the PHY after a Management Read operation or the 16-bit data value to be written to the PHY before a Management Write operation."]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Register Address This field is valid only when C45E is set. It contains the Register Address in the PHY to which the MDIO frame is intended for."]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MII Data This field contains the 16-bit data value read from the PHY after a Management Read operation or the 16-bit data value to be written to the PHY before a Management Write operation."]
    #[inline(always)]
    #[must_use]
    pub fn md(&mut self) -> MD_W<0> {
        MD_W::new(self)
    }
    #[doc = "Bits 16:31 - Register Address This field is valid only when C45E is set. It contains the Register Address in the PHY to which the MDIO frame is intended for."]
    #[inline(always)]
    #[must_use]
    pub fn ra(&mut self) -> RA_W<16> {
        RA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDIO data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macmdiodr](index.html) module"]
pub struct ETH_MACMDIODR_SPEC;
impl crate::RegisterSpec for ETH_MACMDIODR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macmdiodr::R](R) reader structure"]
impl crate::Readable for ETH_MACMDIODR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macmdiodr::W](W) writer structure"]
impl crate::Writable for ETH_MACMDIODR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACMDIODR to value 0"]
impl crate::Resettable for ETH_MACMDIODR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
