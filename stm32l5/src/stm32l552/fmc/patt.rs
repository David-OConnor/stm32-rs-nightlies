#[doc = "Register `PATT` reader"]
pub struct R(crate::R<PATT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PATT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PATT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PATT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PATT` writer"]
pub struct W(crate::W<PATT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PATT_SPEC>;
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
impl From<crate::W<PATT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PATT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATTSET` reader - Attribute memory setup time These bits define the number of KCK_FMC (+1) clock cycles to set up address before the command assertion (NWE, NOE), for NAND Flash read or write access to attribute memory space:"]
pub type ATTSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTSET` writer - Attribute memory setup time These bits define the number of KCK_FMC (+1) clock cycles to set up address before the command assertion (NWE, NOE), for NAND Flash read or write access to attribute memory space:"]
pub type ATTSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PATT_SPEC, u8, u8, 8, O>;
#[doc = "Field `ATTWAIT` reader - Attribute memory wait time These bits define the minimum number of x KCK_FMC (+1) clock cycles to assert the command (NWE, NOE), for NAND Flash read or write access to attribute memory space. The duration for command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of KCK_FMC:"]
pub type ATTWAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTWAIT` writer - Attribute memory wait time These bits define the minimum number of x KCK_FMC (+1) clock cycles to assert the command (NWE, NOE), for NAND Flash read or write access to attribute memory space. The duration for command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of KCK_FMC:"]
pub type ATTWAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PATT_SPEC, u8, u8, 8, O>;
#[doc = "Field `ATTHOLD` reader - Attribute memory hold time These bits define the number of KCK_FMC clock cycles during which the address is held (and data for write access) after the command de-assertion (NWE, NOE), for NAND Flash read or write access to attribute memory space:"]
pub type ATTHOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTHOLD` writer - Attribute memory hold time These bits define the number of KCK_FMC clock cycles during which the address is held (and data for write access) after the command de-assertion (NWE, NOE), for NAND Flash read or write access to attribute memory space:"]
pub type ATTHOLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PATT_SPEC, u8, u8, 8, O>;
#[doc = "Field `ATTHIZ` reader - Attribute memory data bus Hi-Z time These bits define the number of KCK_FMC clock cycles during which the data bus is kept in Hi-Z after the start of a NAND Flash write access to attribute memory space on socket. Only valid for writ transaction:"]
pub type ATTHIZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTHIZ` writer - Attribute memory data bus Hi-Z time These bits define the number of KCK_FMC clock cycles during which the data bus is kept in Hi-Z after the start of a NAND Flash write access to attribute memory space on socket. Only valid for writ transaction:"]
pub type ATTHIZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PATT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Attribute memory setup time These bits define the number of KCK_FMC (+1) clock cycles to set up address before the command assertion (NWE, NOE), for NAND Flash read or write access to attribute memory space:"]
    #[inline(always)]
    pub fn attset(&self) -> ATTSET_R {
        ATTSET_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Attribute memory wait time These bits define the minimum number of x KCK_FMC (+1) clock cycles to assert the command (NWE, NOE), for NAND Flash read or write access to attribute memory space. The duration for command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of KCK_FMC:"]
    #[inline(always)]
    pub fn attwait(&self) -> ATTWAIT_R {
        ATTWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Attribute memory hold time These bits define the number of KCK_FMC clock cycles during which the address is held (and data for write access) after the command de-assertion (NWE, NOE), for NAND Flash read or write access to attribute memory space:"]
    #[inline(always)]
    pub fn atthold(&self) -> ATTHOLD_R {
        ATTHOLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Attribute memory data bus Hi-Z time These bits define the number of KCK_FMC clock cycles during which the data bus is kept in Hi-Z after the start of a NAND Flash write access to attribute memory space on socket. Only valid for writ transaction:"]
    #[inline(always)]
    pub fn atthiz(&self) -> ATTHIZ_R {
        ATTHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Attribute memory setup time These bits define the number of KCK_FMC (+1) clock cycles to set up address before the command assertion (NWE, NOE), for NAND Flash read or write access to attribute memory space:"]
    #[inline(always)]
    #[must_use]
    pub fn attset(&mut self) -> ATTSET_W<0> {
        ATTSET_W::new(self)
    }
    #[doc = "Bits 8:15 - Attribute memory wait time These bits define the minimum number of x KCK_FMC (+1) clock cycles to assert the command (NWE, NOE), for NAND Flash read or write access to attribute memory space. The duration for command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of KCK_FMC:"]
    #[inline(always)]
    #[must_use]
    pub fn attwait(&mut self) -> ATTWAIT_W<8> {
        ATTWAIT_W::new(self)
    }
    #[doc = "Bits 16:23 - Attribute memory hold time These bits define the number of KCK_FMC clock cycles during which the address is held (and data for write access) after the command de-assertion (NWE, NOE), for NAND Flash read or write access to attribute memory space:"]
    #[inline(always)]
    #[must_use]
    pub fn atthold(&mut self) -> ATTHOLD_W<16> {
        ATTHOLD_W::new(self)
    }
    #[doc = "Bits 24:31 - Attribute memory data bus Hi-Z time These bits define the number of KCK_FMC clock cycles during which the data bus is kept in Hi-Z after the start of a NAND Flash write access to attribute memory space on socket. Only valid for writ transaction:"]
    #[inline(always)]
    #[must_use]
    pub fn atthiz(&mut self) -> ATTHIZ_W<24> {
        ATTHIZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The FMC_PATT read/write register contains the timing information for NAND Flash memory bank. It is used for 8-bit accesses to the attribute memory space of the NAND Flash for the last address write access if the timing must differ from that of previous accesses (for Ready/Busy management, refer to Section20.8.5: NAND Flash prewait feature).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [patt](index.html) module"]
pub struct PATT_SPEC;
impl crate::RegisterSpec for PATT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [patt::R](R) reader structure"]
impl crate::Readable for PATT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [patt::W](W) writer structure"]
impl crate::Writable for PATT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PATT to value 0xfcfc_fcfc"]
impl crate::Resettable for PATT_SPEC {
    const RESET_VALUE: Self::Ux = 0xfcfc_fcfc;
}
