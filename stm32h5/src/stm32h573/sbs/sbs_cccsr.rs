#[doc = "Register `SBS_CCCSR` reader"]
pub struct R(crate::R<SBS_CCCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SBS_CCCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SBS_CCCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SBS_CCCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SBS_CCCSR` writer"]
pub struct W(crate::W<SBS_CCCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SBS_CCCSR_SPEC>;
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
impl From<crate::W<SBS_CCCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SBS_CCCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN1` reader - enable compensation cell for VDDIO power rail This bit enables the I/O compensation cell."]
pub type EN1_R = crate::BitReader<bool>;
#[doc = "Field `EN1` writer - enable compensation cell for VDDIO power rail This bit enables the I/O compensation cell."]
pub type EN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SBS_CCCSR_SPEC, bool, O>;
#[doc = "Field `CS1` reader - code selection for VDDIO power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
pub type CS1_R = crate::BitReader<bool>;
#[doc = "Field `CS1` writer - code selection for VDDIO power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
pub type CS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SBS_CCCSR_SPEC, bool, O>;
#[doc = "Field `EN2` reader - enable compensation cell for VDDIO2 power rail This bit enables the I/O compensation cell."]
pub type EN2_R = crate::BitReader<bool>;
#[doc = "Field `EN2` writer - enable compensation cell for VDDIO2 power rail This bit enables the I/O compensation cell."]
pub type EN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SBS_CCCSR_SPEC, bool, O>;
#[doc = "Field `CS2` reader - code selection for VDDIO2 power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
pub type CS2_R = crate::BitReader<bool>;
#[doc = "Field `CS2` writer - code selection for VDDIO2 power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
pub type CS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SBS_CCCSR_SPEC, bool, O>;
#[doc = "Field `RDY1` reader - VDDIO compensation cell ready flag This bit provides the status of the compensation cell."]
pub type RDY1_R = crate::BitReader<bool>;
#[doc = "Field `RDY2` reader - VDDIO2 compensation cell ready flag This bit provides the status of the VDDIO2 compensation cell."]
pub type RDY2_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - enable compensation cell for VDDIO power rail This bit enables the I/O compensation cell."]
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - code selection for VDDIO power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
    #[inline(always)]
    pub fn cs1(&self) -> CS1_R {
        CS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable compensation cell for VDDIO2 power rail This bit enables the I/O compensation cell."]
    #[inline(always)]
    pub fn en2(&self) -> EN2_R {
        EN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - code selection for VDDIO2 power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
    #[inline(always)]
    pub fn cs2(&self) -> CS2_R {
        CS2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - VDDIO compensation cell ready flag This bit provides the status of the compensation cell."]
    #[inline(always)]
    pub fn rdy1(&self) -> RDY1_R {
        RDY1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - VDDIO2 compensation cell ready flag This bit provides the status of the VDDIO2 compensation cell."]
    #[inline(always)]
    pub fn rdy2(&self) -> RDY2_R {
        RDY2_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable compensation cell for VDDIO power rail This bit enables the I/O compensation cell."]
    #[inline(always)]
    #[must_use]
    pub fn en1(&mut self) -> EN1_W<0> {
        EN1_W::new(self)
    }
    #[doc = "Bit 1 - code selection for VDDIO power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
    #[inline(always)]
    #[must_use]
    pub fn cs1(&mut self) -> CS1_W<1> {
        CS1_W::new(self)
    }
    #[doc = "Bit 2 - enable compensation cell for VDDIO2 power rail This bit enables the I/O compensation cell."]
    #[inline(always)]
    #[must_use]
    pub fn en2(&mut self) -> EN2_W<2> {
        EN2_W::new(self)
    }
    #[doc = "Bit 3 - code selection for VDDIO2 power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
    #[inline(always)]
    #[must_use]
    pub fn cs2(&mut self) -> CS2_W<3> {
        CS2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SBS compensation cell for I/Os control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sbs_cccsr](index.html) module"]
pub struct SBS_CCCSR_SPEC;
impl crate::RegisterSpec for SBS_CCCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sbs_cccsr::R](R) reader structure"]
impl crate::Readable for SBS_CCCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sbs_cccsr::W](W) writer structure"]
impl crate::Writable for SBS_CCCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SBS_CCCSR to value 0"]
impl crate::Resettable for SBS_CCCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
